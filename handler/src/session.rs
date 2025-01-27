use kovi::{Message, MsgEvent, PluginBuilder, RuntimeBot};
use rustc_hash::FxHashMap;
use serde::Serialize;
use std::{future::Future, sync::Arc};

use crate::Handler;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SessionId {
    Private(i64),
    Group(i64),
}

pub struct SessionContext {
    bot: Arc<RuntimeBot>,
    id: SessionId,
}

pub trait SessionHandler: Send + Sync + Sized + 'static {
    const NAME: &'static str;
    const VERSION: &'static str;

    fn new(context: SessionContext) -> Self;
    fn context(&self) -> &SessionContext;
    fn bot(&self) -> Arc<RuntimeBot> {
        self.context().bot.clone()
    }
    fn id(&self) -> SessionId {
        self.context().id
    }

    fn send_msg<T>(&self, message: T)
    where
        Message: From<T>,
        T: Serialize,
    {
        match self.id() {
            SessionId::Private(user_id) => {
                self.bot().send_private_msg(user_id, message);
            }
            SessionId::Group(group_id) => {
                self.bot().send_group_msg(group_id, message);
            }
        }
    }

    fn on_msg(&mut self, message: Arc<MsgEvent>) -> impl Future<Output = ()> + Send;
}

pub struct SessionHandlerHost<T: SessionHandler> {
    bot: Arc<RuntimeBot>,
    instances: FxHashMap<SessionId, T>,
}

impl<T: SessionHandler> Handler for SessionHandlerHost<T> {
    const NAME: &'static str = T::NAME;
    const VERSION: &'static str = T::VERSION;

    fn new() -> Self {
        Self {
            bot: PluginBuilder::get_runtime_bot(),
            instances: FxHashMap::default(),
        }
    }

    fn on_msg(&mut self, message: Arc<MsgEvent>) -> impl Future<Output = ()> + Send {
        let id = if let Some(group_id) = message.group_id {
            SessionId::Group(group_id)
        } else {
            SessionId::Private(message.user_id)
        };
        let instance = self.instances.entry(id).or_insert_with(|| {
            T::new(SessionContext {
                bot: self.bot.clone(),
                id,
            })
        });
        instance.on_msg(message)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    pub struct Counter {
        context: SessionContext,
        count: u64,
    }

    impl SessionHandler for Counter {
        const NAME: &'static str = "Counter";
        const VERSION: &'static str = "0.0.1";

        fn new(context: SessionContext) -> Self {
            Self { context, count: 0 }
        }
        fn context(&self) -> &SessionContext {
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
