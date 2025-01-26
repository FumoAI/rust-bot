use kovi::{bot::plugin_builder::event, PluginBuilder as plugin};

#[kovi::plugin]
async fn main() {
    plugin::on_msg(|event| async move {
        if event.borrow_text() == Some("hi") {
            event.reply("Miao~")
        }
    });
}
