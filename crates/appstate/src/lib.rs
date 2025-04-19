use config::Config;
use tokio::sync::Mutex;
use std::sync::Arc;
#[derive(Clone)]
pub struct AppState {
    config: Arc<Mutex<Config>>
}