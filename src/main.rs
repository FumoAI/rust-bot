use kovi::build_bot;

fn main() {
    build_bot!(
        hi,
        ai,
        kovi_plugin_title,
        kovi_plugin_cmd
    ).run();
}
