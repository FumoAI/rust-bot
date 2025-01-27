use kovi::{Message, MsgEvent, PluginBuilder, RuntimeBot};
use rustc_hash::FxHashMap;
use serde::Serialize;
use std::{future::Future, sync::Arc};

use crate::Handler;

pub struct GroupContext {
    bot: Arc<RuntimeBot>,
    group_id: i64,
}

pub trait GroupHandler: Send + Sync + Sized + 'static {
    const NAME: &'static str;
    const VERSION: &'static str;

    fn new(context: GroupContext) -> Self;
    fn context(&self) -> &GroupContext;
    fn bot(&self) -> Arc<RuntimeBot> {
        self.context().bot.clone()
    }
    fn group_id(&self) -> i64 {
        self.context().group_id
    }

    fn send_msg<T>(&self, message: T)
    where
        Message: From<T>,
        T: Serialize,
    {
        self.bot().send_group_msg(self.group_id(), message);
    }

    fn on_msg(&mut self, message: Arc<MsgEvent>) -> impl Future<Output = ()> + Send;
}

pub struct GroupHandlerHost<T: GroupHandler> {
    bot: Arc<RuntimeBot>,
    instances: FxHashMap<i64, T>,
}

impl<T: GroupHandler> Handler for GroupHandlerHost<T> {
    const NAME: &'static str = T::NAME;
    const VERSION: &'static str = T::VERSION;

    fn new() -> Self {
        Self {
            bot: PluginBuilder::get_runtime_bot(),
            instances: FxHashMap::default(),
        }
    }
    fn bot(&self) -> Arc<RuntimeBot> {
        self.bot.clone()
    }

    fn on_group_msg(&mut self, message: Arc<MsgEvent>) -> impl Future<Output = ()> + Send {
        let bot = self.bot();
        let group_id = message.group_id.expect("Message must be from a group");
        let instance = self
            .instances
            .entry(group_id)
            .or_insert_with(|| T::new(GroupContext { bot, group_id }));
        instance.on_msg(message)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    pub struct Counter {
        context: GroupContext,
        count: u64,
    }

    impl GroupHandler for Counter {
        const NAME: &'static str = "Counter";
        const VERSION: &'static str = "0.0.1";

        fn new(context: GroupContext) -> Self {
            Self { context, count: 0 }
        }
        fn context(&self) -> &GroupContext {
            &self.context
        }

        async fn on_msg(&mut self, message: Arc<MsgEvent>) {
            if let Some(text) = message.borrow_text() {
                if text.starts_with("%+") {
                    if let Ok(num) = text[2..].trim().parse::<u64>() {
                        self.count += num;
                        self.send_msg(format!("Counter increased to {}", self.count));
                    } else {
                        self.send_msg("Invalid number");
                    }
                } else if text == "%reset" {
                    self.count = 0;
                    self.send_msg("Counter reset");
                }
            }
        }
    }
}
