use kovi::{tokio::sync::Mutex, Bot, MsgEvent, NoticeEvent, PluginBuilder, RequestEvent};
use std::{future::Future, sync::Arc};

macro_rules! use_listener {
    ($handler: expr, $name: ident) => {
        PluginBuilder::$name({
            let handler = $handler.clone();
            move |e| {
                let handler = handler.clone();
                async move {
                    let mut handler = handler.lock().await;
                    handler.$name(e).await;
                }
            }
        });
    };
}

#[allow(unused_variables)]
pub trait Handler: Send + Sync + Sized + 'static {
    const NAME: &'static str;
    const VERSION: &'static str;

    fn new() -> Self;

    fn mount_on(bot: &mut Bot) {
        kovi::log::info!("Mounting plugin: {}", Self::NAME);

        bot.mount_main(
            Self::NAME,
            Self::VERSION,
            Arc::new(move || Box::pin(setup::<Self>())),
        );

        async fn setup<T: Handler + 'static>() {
            let handler = Arc::new(Mutex::new(T::new()));
            use_listener!(handler, on_msg);
            use_listener!(handler, on_admin_msg);
            use_listener!(handler, on_private_msg);
            use_listener!(handler, on_group_msg);
            use_listener!(handler, on_all_notice);
            use_listener!(handler, on_all_request);
        }
    }

    fn on_msg(&mut self, message: Arc<MsgEvent>) -> impl Future<Output = ()> + Send {
        async {}
    }
    fn on_admin_msg(&mut self, message: Arc<MsgEvent>) -> impl Future<Output = ()> + Send {
        async {}
    }
    fn on_private_msg(&mut self, message: Arc<MsgEvent>) -> impl Future<Output = ()> + Send {
        async {}
    }
    fn on_group_msg(&mut self, message: Arc<MsgEvent>) -> impl Future<Output = ()> + Send {
        async {}
    }
    fn on_all_notice(&mut self, message: Arc<NoticeEvent>) -> impl Future<Output = ()> + Send {
        async {}
    }
    fn on_all_request(&mut self, message: Arc<RequestEvent>) -> impl Future<Output = ()> + Send {
        async {}
    }
    fn drop(&mut self) -> impl Future<Output = ()> + Send {
        async {}
    }
}
