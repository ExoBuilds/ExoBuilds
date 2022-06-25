use crate::database::Database;
use crate::settings::Settings;
use crate::models::match_model::Match;
use crate::models::data_model::Data;
use crate::models::champion_model::Champion;
use crate::utils::serde_json::Map;
use crate::utils::serde_json::Value;
use std::ops::{DerefMut, Deref};
use std::time::Instant;
use ureq::serde_json;

use std::collections::HashSet;

fn retrieve_match(settings: &Settings, puuid: String) -> Result<Vec<String>, ureq::Error> {
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

fn retrieve_matches(settings: &mut Settings, requests: &mut u32) -> HashSet<String> {
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

fn filter_matches(db: &Database, matches: &mut HashSet<String>) {
    let tmp = db.get_matches();
    if tmp.is_err() {
        matches.clear();
        return;
    }
    *matches = &(*matches) - &(tmp.unwrap());
}

fn read_champ(map: &Map<String, Value>) -> Champion {

    Champion {

        id: None,

        win: map.get("win").unwrap().as_bool().unwrap(),
        champion_name: map.get("championName").unwrap().as_str().unwrap().into(),
        kills: map.get("kills").unwrap().as_i64().unwrap().into(),
        assists: map.get("assists").unwrap().as_i64().unwrap().into(),
        deaths: map.get("deaths").unwrap().as_i64().unwrap().into(),
        item0: map.get("item0").unwrap().as_i64().unwrap().into(),
        item1: map.get("item1").unwrap().as_i64().unwrap().into(),
        item2: map.get("item2").unwrap().as_i64().unwrap().into(),
        item3: map.get("item3").unwrap().as_i64().unwrap().into(),
        item4: map.get("item4").unwrap().as_i64().unwrap().into(),
        item5: map.get("item5").unwrap().as_i64().unwrap().into(),
        item6: map.get("item6").unwrap().as_i64().unwrap().into(),
        role: map.get("role").unwrap().as_str().unwrap().into(),

    }

}

fn read_match(settings: &Settings, target: &String) -> Result<Data, ureq::Error> {

    let mut data = Data {
        id: None,
        match_id: target.into(),
        champions: Vec::new()
    };

    let request = format!("https://europe.api.riotgames.com/lol/match/v5/matches/{id}",
        id = target);

    let mut response: serde_json::Value = ureq::get(&request)
                    .set("X-Riot-Token", &settings.riot_api)
                    .call()?
                    .into_json()?;
    Ok({
        if response.as_object().is_some() {
            let response = response.as_object().unwrap();

            let element = response.get("info");

            if element.is_some() && element.unwrap().as_object().is_some() {
                let element = element.unwrap().as_object().unwrap();

                let element = element.get("participants");

                if element.is_some() && element.unwrap().as_array().is_some() {

                    for target in element.unwrap().as_array().unwrap() {
                        if target.as_object().is_some() {
                            data.champions.push(read_champ(target.as_object().unwrap()));
                        }
                    }

                }
            }
    
        }

        data
    })
}

fn read_matches(settings: &Settings, matches: &mut HashSet<String>, requests: &mut u32, clock: &mut Instant) -> HashSet<Data> {
    let mut data: HashSet<Data> = HashSet::new();
    let mut size = matches.len();

    while matches.len() > 0 {

        if clock.elapsed().as_secs() > 120 {
            *clock = Instant::now();
            *requests = 80;
        }

        if *requests == 0 {
            continue;
        }

        let element = matches.iter().next().cloned().unwrap();
        matches.take(&element).unwrap();

        let target = read_match(settings, &element);
        if target.is_err() {
            continue;
        }

        data.insert(target.unwrap());

        *requests -= 1_u32;
    }

    data
}

fn publish_matches(db: &Database, matches: &HashSet<String>) {
    db.add_matches(matches.into_iter().collect());
}

fn publish_data(db: &Database, data: HashSet<Data>) {
    db.add_data(data.into_iter().collect());
}

pub fn initialize_matches(settings: &mut Settings, db: &Database) {
    let mut clock = Instant::now();
    let mut requests: u32 = 80;

    loop {
        if clock.elapsed().as_secs() > 120 {
            clock = Instant::now();
            requests = 80;
        }

        if requests == 0 {
            continue;
        }

        let mut matches = retrieve_matches(settings, &mut requests);
        if matches.is_empty() {
            continue;
        }
        filter_matches(db, &mut matches);
        if matches.is_empty() {
            continue;
        }

        publish_matches(db, &matches);
        let data = read_matches(settings, &mut matches, &mut requests, &mut clock);
        publish_data(&db, data);
    }
}
