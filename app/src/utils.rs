use crate::database::Database;
use crate::settings::Settings;
use crate::models::match_model::Match;
use crate::models::data_model::Data;
use std::ops::{DerefMut, Deref};
use std::time::Instant;
use ureq::serde_json;

use std::collections::HashSet;

pub fn retrieve_match(settings: &Settings, puuid: String) -> Result<Vec<String>, ureq::Error> {
    let mut matches: Vec<String> = Vec::new();

    let request = format!("https://europe.api.riotgames.com/lol/match/v5/matches/by-puuid/{puuid}/{ids}",
    puuid = &puuid,
    ids = "ids?start=0&count=100");

    let mut response: serde_json::Value = ureq::get(&request)
                    .set("X-Riot-Token", &settings.riot_api)
                    .call()?
                    .into_json()?;
    Ok({
        if response.as_array_mut().is_some() {
            let response = response.as_array_mut().unwrap();
    
            for element in response.to_vec().into_iter() {
                matches.push(element.as_str().unwrap().into());
            }
        }

        matches
    })
}

pub fn retrieve_matches(settings: &mut Settings, requests: &mut u32) -> HashSet<String> {
    let mut matches = HashSet::new();
    let mut size = settings.puuid.len();

    while *requests >= 40 && size > 0 {

            if settings.puuid.is_empty() {
                return matches;
            }

            let puuid = settings.puuid.pop_front().unwrap();
            settings.puuid.push_back(puuid.clone());

            let target = retrieve_match(settings, puuid);

            if target.is_err() {
                *requests = 0;
                continue;
            }

            let target: HashSet<String> = target.unwrap().into_iter().collect();

            matches.extend(target);

            *requests -= 1_u32;
            size -= 1;
        }

    matches
}

pub fn filter_matches(db: &Database, matches: &mut HashSet<String>) {
    let tmp = db.get_matches();
    if tmp.is_err() {
        matches.clear();
        return;
    }
    *matches = &(*matches) - &(tmp.unwrap());
}

pub fn read_matches(settings: &Settings, matches: &HashSet<String>) -> HashSet<Data> {
    let mut data = HashSet::new();

    data
}

pub fn publish_matches(db: &Database, matches: HashSet<String>) {
    db.add_matches(matches.into_iter().collect());
}

pub fn publish_data(db: &Database, data: HashSet<Data>) {
    db.add_data(data.into_iter().collect());
}

pub fn initialize_matches(settings: &mut Settings, db: &Database) {
    let mut start = Instant::now();
    let mut requests: u32 = 80;

    loop {
        if start.elapsed().as_secs() > 120 {
            start = Instant::now();
            requests = 80;
        }

        if requests == 0 {
            continue;
        }

        let mut matches = retrieve_matches(settings, &mut requests);
        filter_matches(db, &mut matches);
        if (matches.is_empty()) {
            continue;
        }
        let data = read_matches(settings, &matches);
        publish_matches(db, matches);
        //publish_data(&db, data);
    }
}
