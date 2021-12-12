pub use anyhow::Result as AnyResult;
pub use futures::{Future, FutureExt, TryFutureExt, TryStreamExt};
pub use nats::asynk::{Message as NatsMessage, Options as NatsOptions};

use async_trait::async_trait;
use blocking::unblock;
use bytes::Bytes;
use log::{debug, error};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

#[async_trait]
pub trait NatsMessageHandler: Sync + Send {
    async fn handle_message(&mut self, message: &NatsMessage) -> AnyResult<ProcessResult>;
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
        let connection = options.connect(&self.url).await?;
        connection.flush_timeout(self.timeout).await?;
        let subscription = connection.queue_subscribe(&self.source_topic, &self.group).await?;

        while !shutdown_flag.load(Ordering::Relaxed) {
            if let Some(received_message) = subscription.try_next() {
                match on_message.handle_message(&received_message).await {
                    Err(error) => error!("{}", error),
                    Ok(process_result) => match process_result {
                        ProcessResult::Failure(failure_message) => {
                            let message_len = failure_message.len();

                            if let Err(error) =
                                connection.publish(&self.failure_topic, &failure_message).await
                            {
                                error!("{}", error);
                                continue;
                            }

                            debug!(
                                "Failure message sent to topic {} with size {} bytes",
                                &self.failure_topic, message_len
                            );
                        }
                        ProcessResult::Success(success_message) => {
                            let message_len = success_message.len();

                            if let Err(error) =
                                connection.publish(&self.success_topic, &success_message).await
                            {
                                error!("{}", error);
                                continue;
                            }

                            debug!(
                                "Success message sent to topic {} with size {} bytes",
                                &self.success_topic, message_len
                            );
                        }
                    },
                }

                continue;
            }

            let sleep_time = self.timeout;
            unblock(move || {
                sleep(sleep_time);
            })
            .await;
        }

        debug!("Exiting...");

        if self.unsub_on_exit {
            subscription.unsubscribe().await?;
        }

        Ok(())
    }
}
