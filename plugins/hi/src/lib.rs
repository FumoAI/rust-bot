use kovi::PluginBuilder as plugin;
use rand::random as random;

#[kovi::plugin]
async fn main() {
    plugin::on_msg(|event| async move {
        if event.borrow_text() == Some("摸摸") {
            if random::<f32>() < 0.75 {
                event.reply("喵呜~");
            } else {
                event.reply("( *・ω・)✄╰ひ╯");
            }
        }
    });
}
