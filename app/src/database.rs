use crate::settings::Settings;
use std::collections::HashSet;

use mongodb::{
    bson::{extjson::de::Error},
    results::{InsertOneResult, InsertManyResult},
    sync::{Client, Collection},
};

use crate::models::{match_model::Match, data_model::Data};

pub struct Database {
    pub matches: Collection<Match>,
    pub data: Collection<Data>,
}

impl Database {
    pub fn init(settings: &Settings) -> Self {
        let uri = settings.mongo_uri.clone();
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("exobuilds");
        let matches: Collection<Match> = db.collection("match");
        let data: Collection<Data> = db.collection("data");
        Database { 
            matches: matches,
            data: data,
        }
    }

    pub fn get_matches(&self) -> Result<HashSet<Match>, Error> {
        let mut elements: HashSet<Match> = HashSet::new();
        let matches = self.
            matches
            .find(None, None)
            .ok()
            .expect("Error whilst retrieve every matches");
        for target in matches.into_iter() {
            if target.is_err() {
                continue;
            }
            elements.insert(target.unwrap());
        }
        Ok(elements)
    }

    pub fn create_match(&self, new_match: Match) -> Result<InsertOneResult, Error> {
        let new_doc = Match {
            id: None,
            match_id: new_match.match_id,
        };
        let target = self
            .matches
            .insert_one(new_doc, None)
            .ok()
            .expect("Error whilst adding new match");
        Ok(target)
    }

    pub fn add_matches(&self, new_matches: Vec<Match>) -> Result<InsertManyResult, Error> {
        let target = self
            .matches
            .insert_many(new_matches, None)
            .ok()
            .expect("Error whilst adding new matches");
        Ok(target)
    }

    pub fn add_data(&self, new_data: Vec<Data>) -> Result<InsertManyResult, Error> {
        let target = self
            .data
            .insert_many(new_data, None)
            .ok()
            .expect("Error whilst adding new data");
        Ok(target)
    }

    pub fn init_match_data(&self, target: Match) -> Result<InsertOneResult, Error> {
        let champions = Vec::new();
        let new_doc = Data {
            id: None,
            match_id: target.match_id,
            champions: champions,
        };
        let target = self
            .data
            .insert_one(new_doc, None)
            .ok()
            .expect("Error whilst adding new data");
        Ok(target)
    }

}