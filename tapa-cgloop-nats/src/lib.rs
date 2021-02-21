pub use anyhow::Result as AnyResult;
pub use async_nats::{Message as NatsMessage, Options as NatsOptions};
pub use futures::Future;

use async_trait::async_trait;
use bytes::Bytes;
use log::debug;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub enum ProcessResult {
    Success(Bytes),
    Failure(Bytes),
}

#[async_trait]
pub trait NatsMessageHandler: Sync {
    async fn handle_message<'a>(&self, message: &'a NatsMessage) -> AnyResult<ProcessResult>;
}

pub struct CGLoop {
    pub url: String,
    pub source_topic: String,
    pub success_topic: String,
    pub failure_topic: String,
    pub group: String,
    pub unsub_on_exit: bool,
}

impl CGLoop {
    pub fn new(
        url: &str,
        source_topic: &str,
        success_topic: &str,
        failure_topic: &str,
        group: &str,
        unsub_on_exit: bool,
    ) -> Self {
        Self {
            url: url.into(),
            source_topic: source_topic.into(),
            success_topic: success_topic.into(),
            failure_topic: failure_topic.into(),
            group: group.into(),
            unsub_on_exit,
        }
    }

    pub async fn run(
        self,
        options: NatsOptions,
        shutdown_flag: Arc<AtomicBool>,
        on_message: &dyn NatsMessageHandler,
    ) -> AnyResult<()> {
        let connection = options.connect(&self.url).await?;
        let subscription = connection.queue_subscribe(&self.source_topic, &self.group).await?;

        while !shutdown_flag.load(Ordering::Relaxed) {
            if let Some(received_message) = subscription.next().await {
                match on_message.handle_message(&received_message).await? {
                    ProcessResult::Failure(failure_message) => {
                        connection.publish(&self.failure_topic, &failure_message).await?;
                        debug!(
                            "Failure message sent to topic {} with size {} bytes",
                            &self.failure_topic,
                            failure_message.len()
                        );
                    }
                    ProcessResult::Success(success_message) => {
                        connection.publish(&self.success_topic, &success_message).await?;
                        debug!(
                            "Failure message sent to topic {} with size {} bytes",
                            &self.success_topic,
                            success_message.len()
                        );
                    }
                }
            }
        }

        debug!("Loop exited");

        Ok(())
    }
}
