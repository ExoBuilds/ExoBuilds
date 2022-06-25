use crate::{settings::Settings, models::{recommended_champion_model::RecommendedChampion, data_model::Data}};
use std::collections::HashSet;

use mongodb::{
    bson::extjson::de::Error,
    results::InsertManyResult,
    sync::{Client, Collection},
};

pub struct Database {
    pub data: Collection<Data>,
    pub champion: Collection<RecommendedChampion>
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Database {
            data: self.data.clone_with_type(),
            champion: self.champion.clone_with_type()
        }
    }
}

impl Database {
    pub fn init(settings: &Settings) -> Self {
        let uri = settings.mongo_uri.clone();
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("exobuilds");
        Database {
            data: db.collection("data"),
            champion: db.collection("champions")
        }
    }

    pub fn get_matches(&self) -> Result<HashSet<String>, Error> {
        let mut elements: HashSet<String> = HashSet::new();
        let data = self.
            data
            .find(None, None)
            .ok()
            .expect("Error whilst retrieve every matches");
        for target in data.into_iter() {
            if target.is_err() {
                continue;
            }
            elements.insert(target.unwrap().match_id);
        }
        Ok(elements)
    }

    pub fn add_data(&self, new_data: Vec<Data>) -> Result<InsertManyResult, Error> {
        let target = self
            .data
            .insert_many(new_data, None)
            .ok()
            .expect("Error whilst adding new data");
        Ok(target)
    }

}
