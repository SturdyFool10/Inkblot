use config::{Config, load_config};

#[tokio::main]
async fn main() {
    let config: Config = load_config("config.json");
    println!("{:#?}", config);
}
