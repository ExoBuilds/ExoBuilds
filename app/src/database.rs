use crate::{
    models::{
        champion_model::Champion, data_model::Data, match_history_model::MatchHistory,
        recommended_champion_model::RecommendedChampion,
    },
    settings::Settings,
};
use std::collections::{HashMap, HashSet};

use mongodb::{
    bson::{doc, extjson::de::Error},
    options::ReplaceOptions,
    results::{InsertOneResult, UpdateResult},
    sync::{Client, Collection},
};

pub struct Database {
    pub data: Collection<Data>,
    pub champion: Collection<RecommendedChampion>,
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Database {
            data: self.data.clone_with_type(),
            champion: self.champion.clone_with_type(),
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
            champion: db.collection("champions"),
        }
    }

    pub fn get_matches(&self) -> Result<HashSet<String>, Error> {
        let mut elements: HashSet<String> = HashSet::new();
        let data = self
            .data
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

    pub fn get_champions(&self) -> Result<HashMap<String, Vec<Champion>>, Error> {
        let mut elements: HashMap<String, Vec<Champion>> = HashMap::new();
        let data = self
            .data
            .find(None, None)
            .ok()
            .expect("Error whilst retrieve every matches");
        for target in data.into_iter() {
            if target.is_err() {
                continue;
            }
            for champ in target.unwrap().champions {
                elements
                    .entry(champ.champion_name.clone())
                    .or_insert_with(Vec::new)
                    .push(champ);
            }
        }
        Ok(elements)
    }

    pub fn update_recommended_champion(
        &self,
        value: RecommendedChampion,
    ) -> Result<UpdateResult, Error> {
        let target = self
            .champion
            .replace_one(
                doc! {"role": value.role.clone(), "name": value.name.clone()},
                value,
                ReplaceOptions::builder().upsert(true).build(),
            )
            .ok()
            .expect("Error whilst adding new data");
        Ok(target)
    }

    pub fn get_recommended_champion(&self, name: &str) -> Result<RecommendedChampion, Error> {
        let mut result = RecommendedChampion::default();
        let mut samples: i64 = -1;
        let data = self
            .champion
            .find(doc! {"name": name}, None)
            .ok()
            .expect("Error whilst retrieve every matches");
        for target in data.into_iter() {
            if target.is_err() {
                continue;
            }
            let tmp = target.unwrap();

            if tmp.wins + tmp.loses > samples {
                samples = tmp.wins + tmp.loses;
                result = tmp;
            }
        }
        Ok(result)
    }

    pub fn add_data(&self, new_data: Data) -> Result<InsertOneResult, Error> {
        let target = self
            .data
            .insert_one(new_data, None)
            .ok()
            .expect("Error whilst adding new data");
        Ok(target)
    }

    pub fn get_player_matches(&self, puuid: &str) -> Result<Vec<MatchHistory>, Error> {
        let mut result: Vec<MatchHistory> = Vec::new();
        let data = self
            .data
            .find(doc! {"champions.puuid": puuid}, None)
            .ok()
            .expect("Error whilst retrieve player matches");
        for target in data.into_iter() {
            if target.is_err() {
                continue;
            }
            let tmp_data = target.unwrap();
            let mut player_champion: Champion = Champion::default();
            for champ in &tmp_data.champions {
                if champ.puuid == puuid {
                    player_champion = champ.clone();
                }
            }
            result.push(MatchHistory {
                player_champion: player_champion,
                data: tmp_data,
            });
        }
        result.sort_by(|a, b| a.data.match_creation.cmp(&b.data.match_creation));
        result.reverse();
        Ok(result)
    }
}
