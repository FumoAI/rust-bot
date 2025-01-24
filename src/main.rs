use kovi::build_bot;

fn main() {
    for entry in std::fs::read_dir(".").unwrap() {
        let entry = entry.unwrap();
        println!("{:?}", entry.path());
    }
    // show content in the kovi.conf.toml file
    // let content = std::fs::read_to_string("kovi.conf.toml").unwrap();
    // println!("{}", content);
    build_bot!(hi).run();
}
