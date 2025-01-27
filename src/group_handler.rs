use kovi::{tokio::sync::Mutex, Bot, Message, MsgEvent, PluginBuilder, RuntimeBot};
use rustc_hash::FxHashMap;
use serde::Serialize;
use std::{future::Future, sync::Arc};

pub struct GroupContext {
    bot: Arc<RuntimeBot>,
    group_id: i64,
}

impl GroupContext {
    pub fn send_msg<T>(&self, message: T)
    where
        Message: From<T>,
        T: Serialize,
    {
        self.bot.send_group_msg(self.group_id, message);
    }
}

pub trait GroupHandler: Send + Sync {
    fn name() -> &'static str {
        "<unnamed>"
    }

    fn version() -> &'static str {
        "0.0.1"
    }

    fn new(context: GroupContext) -> Self
    where
        Self: Sized;

    fn on_msg(&mut self, message: Arc<MsgEvent>) -> impl Future<Output = ()> + Send;
}

pub fn mount_group_handler<T: GroupHandler + 'static>(bot: &mut Bot) {
    async fn setup<T: GroupHandler + 'static>() {
        let bot = PluginBuilder::get_runtime_bot();
        let instances = Arc::new(Mutex::new(FxHashMap::<i64, Arc<Mutex<T>>>::default()));

        PluginBuilder::on_group_msg(move |e| {
            let bot = bot.clone();
            let instances = instances.clone();
            async move {
                let group_id = e.group_id.expect("Message must be from a group");
                let instance = {
                    let mut instances = instances.lock().await;
                    instances
                        .entry(group_id)
                        .or_insert_with(|| {
                            let context = GroupContext { bot, group_id };
                            Arc::new(Mutex::new(T::new(context)))
                        })
                        .clone()
                };

                let mut instance = instance.lock().await;
                instance.on_msg(e).await;
            }
        });
    }

    bot.mount_main(
        T::name(),
        T::version(),
        Arc::new(move || Box::pin(setup::<T>())),
    )
}

pub struct Counter {
    context: GroupContext,
    count: u64,
}

impl GroupHandler for Counter {
    fn name() -> &'static str {
        "Counter"
    }

    fn version() -> &'static str {
        "0.0.1"
    }

    fn new(context: GroupContext) -> Self {
        Self { context, count: 0 }
    }

    async fn on_msg(&mut self, message: Arc<MsgEvent>) {
        if let Some(text) = message.borrow_text() {
            if text.starts_with("%+") {
                if let Ok(num) = text[2..].trim().parse::<u64>() {
                    self.count += num;
                    self.context
                        .send_msg(format!("Counter increased to {}", self.count));
                } else {
                    self.context.send_msg("Invalid number");
                }
            } else if text == "%reset" {
                self.count = 0;
                self.context.send_msg("Counter reset");
            }
        }
    }
}
