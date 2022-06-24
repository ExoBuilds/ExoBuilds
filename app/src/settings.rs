use std::env;
extern crate dotenv;
use dotenv::dotenv;

use std::collections::VecDeque;

pub struct Settings {
    pub mongo_uri: String,
    pub puuid: VecDeque<String>,
    pub riot_api: String
}

impl Settings {
    pub fn init() -> Self {
        dotenv().ok();
        let mongo_uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let puuid = match env::var("PUUID") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let riot_api = match env::var("RIOT_API") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        Self {
            mongo_uri: mongo_uri,
            puuid: puuid.split(":").map(|s| s.into()).collect(),
            riot_api: riot_api
        }
    }
}