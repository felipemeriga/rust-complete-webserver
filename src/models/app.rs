use crate::configuration::config::load_default;
use crate::configuration::config::Config;
use crate::database::mongodb_repo::MongoRepo;
use crate::database::repository::Repository;
use std::sync::Arc;

#[derive(Debug)]
pub struct AppData {
    pub db: Arc<dyn Repository>,
    pub config: Config,
}

impl AppData {
    pub(crate) async fn init() -> AppData {
        let config = load_default().expect("error getting configuration.yaml file from ./");
        let mongo_repo = MongoRepo::init(config.clone().mongo_uri).await;

        AppData {
            db: Arc::new(mongo_repo),
            config,
        }
    }
}
