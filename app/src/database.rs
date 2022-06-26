use crate::{settings::Settings, models::{recommended_champion_model::{RecommendedChampion}, data_model::Data, champion_model::Champion}};
use std::collections::{HashSet, HashMap};

use mongodb::{
    bson::{extjson::de::Error, doc},
    results::{UpdateResult, InsertOneResult},
    sync::{Client, Collection}, options::ReplaceOptions,
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

    pub fn get_champions(&self) -> Result<HashMap<String, Vec<Champion>>, Error> {
        let mut elements: HashMap<String, Vec<Champion>> = HashMap::new();
        let data = self.
            data
            .find(None, None)
            .ok()
            .expect("Error whilst retrieve every matches");
        for target in data.into_iter() {
            if target.is_err() {
                continue;
            }
            for champ in target.unwrap().champions {
                elements.entry(champ.champion_name.clone())
                .or_insert_with(Vec::new)
                .push(champ);
            }
        }
        Ok(elements)
    }

    pub fn update_recommended_champion(&self, value: RecommendedChampion) -> Result<UpdateResult, Error> {
        
        let target = self
            .champion
            .replace_one(doc! {"role": value.role.clone(), "name": value.name.clone()}, value, ReplaceOptions::builder().upsert(true).build())
            .ok()
            .expect("Error whilst adding new data");
        Ok(target)

    }

    pub fn get_recommended_champion(&self, name: &str) -> Result<RecommendedChampion, Error> {
        let mut result = RecommendedChampion::default();
        let mut samples: i64 = -1;
        let data = self.
            champion
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

}
