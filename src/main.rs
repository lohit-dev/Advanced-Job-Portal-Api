use e_commerce::config::Config;

fn main() {
    let config = Config::load();
    println!("{:#?}", config);
}
