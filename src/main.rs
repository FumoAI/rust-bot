use kovi::build_bot;
use rust_bot::group_handler::{Counter, GroupHandler};

fn main() {
    let mut bot = build_bot!(hi, ai, kovi_plugin_title, kovi_plugin_cmd);
    Counter::mount_on(&mut bot);
    bot.run();
}
