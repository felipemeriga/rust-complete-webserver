use crate::auth::{api_key::ApiKeyData, auth0::Auth0Data};
use crate::configuration::prelude::Result as AppResult;
use serde::Serialize;
use std::path::PathBuf;
use twelf::{config, Layer};

const CONFIG_FILE: &str = "./config.yaml";

#[allow(unused)]
pub fn load(path: &str) -> AppResult<Config> {
    let conf = Config::with_layers(&[Layer::Yaml(PathBuf::from(path))])?;
    Ok(conf)
}

pub fn load_default() -> AppResult<Config> {
    let conf = Config::with_layers(&[Layer::Yaml(PathBuf::from(CONFIG_FILE))])?;
    Ok(conf)
}

#[config]
#[derive(Debug, Default, Serialize, Clone)]
pub struct Config {
    pub env: String,
    pub api_key_data: ApiKeyData,
    pub auth0_data: Auth0Data,
    pub mongo_uri: String,
}

// This struct simulates a S3 bucket, or a data storage, and we are using Arc, because we need multiple ownership,
// and we want to simulate a thread that keeps fetching data from the bucket, to see if something changed, and
// persists the change on the Arc<Mutex<String>>
// #[derive(Debug, Clone)]
// pub struct BucketConfig {
//     pub data: Arc<Mutex<String>>,
// }

// impl BucketConfig {
//     pub fn init() -> Self {
//         BucketConfig {
//             data: Arc::new(Mutex::new(String::new())),
//         }
//     }
//
//     // This function is used to simulate a background process for updating the configuration in the bucket,
//     // We are using Arc, since we need multiple ownership with thread safe.
//     fn watch_bucket_changes(&mut self) {
//         // creating a local self, because we can't pass &mut self to the thread function, since the ownership of
//         // &mut self belongs to the function, and the struct is already borrowing it to the function, so you can't move it to the thread
//         let local_self = self.data.clone();
//
//         // thread::spawn method receives a closure, which is a function, and we need to use move keyword, because if we look inside
//         // the thread closure, we are using local_self, which is owned by the parent function, we need to use move keyword, to move the
//         // ownership to the thread, for example if after the thread::spawn we try to use the value of local_self, we will get an
//         // ownership error
//         // also, we can't use &mut self, because it will escape the function, and it has been already borrowed by the parent function
//         thread::spawn(move || {
//             let mut counter: u32 = 0;
//             loop {
//                 println!("detected configuration change");
//                 local_self.lock().unwrap().push_str(&(" ".to_owned() + &*counter.to_string()));
//                 thread::sleep(Duration::from_millis(20000));
//                 counter += 1;
//             }
//         });
//     }
// }
