use kovi::build_bot;
use rust_bot::group_handler::{mount_group_handler, Counter};

fn main() {
    let mut bot = build_bot!(hi, ai, kovi_plugin_title, kovi_plugin_cmd);
    mount_group_handler::<Counter>(&mut bot);
    bot.run();
}
