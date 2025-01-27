use kovi::PluginBuilder as plugin;
use rand::random as random;

#[kovi::plugin]
async fn main() {
    plugin::on_msg(|event| async move {
        if event.borrow_text() == Some("摸摸") {
            if random::<f32>() < 0.75 {
                if let Some(nickname) = event.sender.nickname.as_ref() {
                    event.reply(format!("摸摸 {}", nickname));
                } else {
                    event.reply("喵呜~");
                }
            } else {
                event.reply("( *・ω・)✄╰ひ╯");
            }
        }
    });
}
