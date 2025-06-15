use e_commerce::config::Config;

fn main() {
    let config = Config::load();
    // let state = build_state(config);
    println!("{:#?}", config);
}
