use crate::database::Database;
use crate::settings::Settings;
use crate::models::match_model::Match;
use crate::models::data_model::Data;
use std::ops::{DerefMut, Deref};
use std::time::Instant;
use ureq::serde_json;

use std::collections::HashSet;

pub fn retrieve_match(settings: &Settings, puuid: String) -> Result<Vec<Match>, ureq::Error> {
    let mut matches: Vec<Match> = Vec::new();

    let request = format!("https://europe.api.riotgames.com/lol/match/v5/matches/by-puuid/{puuid}/{ids}",
    puuid = &puuid,
    ids = "ids?start=0&count=100");

    let response: serde_json::Value = ureq::get(&request)
                    .set("X-Riot-Token", &settings.riot_api)
                    .call()?
                    .into_json()?;

    for match_id in response.as_array().into_iter() {
        matches.push(Match {id: None, match_id: serde_json::to_string(match_id).unwrap()});
    }

    Ok(matches)
}

pub fn retrieve_matches(settings: &mut Settings, requests: &mut u32) -> HashSet<Match> {
    let mut matches = HashSet::new();

    while *requests >= 40 {

            if settings.puuid.is_empty() {
                return matches;
            }

            let puuid = settings.puuid.pop().unwrap();
            settings.puuid.push(puuid.clone());

            let target = retrieve_match(settings, puuid);

            if target.is_err() {
                *requests = 0;
                continue;
            }

            let target: HashSet<Match> = target.unwrap().into_iter().collect();

            matches.extend(target);

            *requests -= 1_u32;
        }

    matches
}

pub fn filter_matches(db: &Database, matches: &mut HashSet<Match>) {
    let tmp = db.get_matches();
    if tmp.is_err() {
        matches.clear();
        return;
    }
    *matches = &(*matches) - &(tmp.unwrap());
}

pub fn read_matches(settings: &Settings, matches: &HashSet<Match>) -> HashSet<Data> {
    let mut data = HashSet::new();

    data
}

pub async fn publish_matches(db: &Database, matches: HashSet<Match>) {
    db.add_matches(matches.into_iter().collect());
}

pub async fn publish_data(db: &Database, data: HashSet<Data>) {
    db.add_data(data.into_iter().collect());
}

pub fn initialize_matches(settings: &mut Settings, db: &Database) {
    let mut start = Instant::now();
    let mut requests: u32 = 80;

    loop {
        if start.elapsed().as_secs() > 120 {
            start = Instant::now();
            requests = 80;
            println!("reset!");
        }

        if requests == 0 {
            println!("waiting for more requests");
            continue;
        }

        let mut matches = retrieve_matches(settings, &mut requests);
        filter_matches(db, &mut matches);
        let data = read_matches(settings, &matches);
        publish_matches(db, matches);
        publish_data(db, data);
    }
}
