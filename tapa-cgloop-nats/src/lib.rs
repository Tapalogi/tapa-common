pub use anyhow::Result as AnyResult;
pub use futures::{Future, FutureExt, TryFutureExt, TryStreamExt};
pub use nats::{Message as NatsMessage, Options as NatsOptions};

use blocking::unblock;
use bytes::Bytes;
use log::{debug, error};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub trait NatsMessageHandler: Sync + Send {
    fn handle_message(&mut self, message: &NatsMessage) -> AnyResult<ProcessResult>;
}

pub enum ProcessResult {
    Success(Bytes),
    Failure(Bytes),
}

pub struct CGLoop {
    pub url: String,
    pub source_topic: String,
    pub success_topic: String,
    pub failure_topic: String,
    pub group: String,
    pub unsub_on_exit: bool,
    pub timeout: Duration,
}

impl CGLoop {
    pub fn new(
        url: &str,
        source_topic: &str,
        success_topic: &str,
        failure_topic: &str,
        group: &str,
        unsub_on_exit: bool,
        timeout: Option<Duration>,
    ) -> Self {
        let timeout = if let Some(timeout_value) = timeout {
            timeout_value
        } else {
            Duration::from_millis(500)
        };

        Self {
            url: url.into(),
            source_topic: source_topic.into(),
            success_topic: success_topic.into(),
            failure_topic: failure_topic.into(),
            group: group.into(),
            unsub_on_exit,
            timeout,
        }
    }

    pub async fn run(
        self,
        options: NatsOptions,
        shutdown_flag: Arc<AtomicBool>,
        on_message: Box<dyn NatsMessageHandler>,
    ) -> AnyResult<()> {
        let mut on_message = on_message;

        let _ = unblock(move || {
            let connection;

            if let Ok(new_connection) = options.connect(&self.url) {
                connection = new_connection;
            } else {
                error!("Unable to connect to {}", &self.url);
                panic!();
            }

            if connection.flush_timeout(self.timeout).is_err() {
                error!("Unable to set flush timeout to {:#?}", self.timeout);
                panic!();
            }

            let subscription;

            if let Ok(queue_subscription) =
                connection.queue_subscribe(&self.source_topic, &self.group)
            {
                subscription = queue_subscription;
            } else {
                error!(
                    "Unable to do queue subscription to subject {} queue {}",
                    &self.source_topic, &self.group
                );
                panic!();
            }

            while !shutdown_flag.load(Ordering::Relaxed) {
                if let Ok(received_message) = subscription.next_timeout(self.timeout) {
                    match on_message.handle_message(&received_message) {
                        Err(error) => {
                            error!("{}", error);
                        }
                        Ok(process_result) => match process_result {
                            ProcessResult::Failure(failure_message) => {
                                let message_len = failure_message.len();

                                if let Err(error) =
                                    connection.publish(&self.failure_topic, &failure_message)
                                {
                                    error!("{}", error);
                                } else {
                                    debug!(
                                        "Failure message sent to topic {} with size {} bytes",
                                        &self.failure_topic, message_len
                                    );
                                }
                            }
                            ProcessResult::Success(success_message) => {
                                let message_len = success_message.len();

                                if let Err(error) =
                                    connection.publish(&self.success_topic, &success_message)
                                {
                                    error!("{}", error);
                                } else {
                                    debug!(
                                        "Success message sent to topic {} with size {} bytes",
                                        &self.success_topic, message_len
                                    );
                                }
                            }
                        },
                    }
                }
            }

            debug!("Exiting...");

            if self.unsub_on_exit {
                subscription.unsubscribe().unwrap();
            }
        })
        .await;

        Ok(())
    }
}
