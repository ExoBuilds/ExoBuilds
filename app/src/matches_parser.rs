use crate::database::Database;
use crate::settings::Settings;
use crate::models::data_model::Data;
use crate::models::champion_model::Champion;
use crate::matches_parser::serde_json::Map;
use crate::matches_parser::serde_json::Value;
use std::time::Instant;
use ureq::serde_json;

use std::collections::HashSet;

fn retrieve_match(settings: &Settings, puuid: &String) -> Result<Vec<String>, ureq::Error> {
    let mut matches: Vec<String> = Vec::new();

    let request = format!("https://europe.api.riotgames.com/lol/match/v5/matches/by-puuid/{puuid}/{ids}",
        puuid = puuid,
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

            let target = retrieve_match(settings, &puuid);
            settings.puuid.push_back(puuid);

            if target.is_err() {
                *requests = 0;
                continue;
            }

            let target: HashSet<String> = target.unwrap().into_iter().collect();

            matches.extend(target);

            *requests -= 1;
            size -= 1;
        }

    matches
}

fn filter_matches(db: &Database, matches: &mut HashSet<String>) {
    let tmp = db.get_matches();
    if tmp.is_err() {
        return;
    }
    *matches = &(*matches) - &(tmp.unwrap());
}

fn read_champ(map: &Map<String, Value>) -> Champion {

    let q = map.get("spell1Casts").unwrap().as_i64().unwrap();
    let w = map.get("spell2Casts").unwrap().as_i64().unwrap();
    let e = map.get("spell3Casts").unwrap().as_i64().unwrap();
    let r = map.get("spell4Casts").unwrap().as_i64().unwrap();


    let mut spells = vec![q, w, e, r];

    spells.sort();
    spells.reverse();

    let spellmax1: String = match spells.get(0).unwrap() {
        target if target == &q => "Q".into(),
        target if target == &w => "W".into(),
        target if target == &e => "E".into(),
        target if target == &r => "R".into(),
        _ => "".into()
    };

    spells.remove(0);

    let spellmax2: String = match spells.get(0).unwrap() {
        target if target == &q => "Q".into(),
        target if target == &w => "W".into(),
        target if target == &e => "E".into(),
        target if target == &r => "R".into(),
        _ => "".into()
    };

    spells.remove(0);

    let spellmax3: String = match spells.get(0).unwrap() {
        target if target == &q => "Q".into(),
        target if target == &w => "W".into(),
        target if target == &e => "E".into(),
        target if target == &r => "R".into(),
        _ => "".into()
    };

    spells.remove(0);

    let spellmax4: String = match spells.get(0).unwrap() {
        target if target == &q => "Q".into(),
        target if target == &w => "W".into(),
        target if target == &e => "E".into(),
        target if target == &r => "R".into(),
        _ => "".into()
    };

    let summoner1: String = match map.get("summoner1Id").unwrap().as_i64().unwrap() {
        1 => "Boost".into(),
        3 => "Exhaust".into(),
        4 => "Flash".into(),
        6 => "Haste".into(),
        7 => "Heal".into(),
        11 => "Smite".into(),
        12 => "Teleport".into(),
        13 => "Mana".into(),
        14 => "Dot".into(),
        21 => "Barrier".into(),
        30 => "PoroRecall".into(),
        31 => "PoroThrow".into(),
        32 => "Snowball".into(),
        39 => "SnowURFSnowball_Mark".into(),
        54 => "_UltBookPlaceholder".into(),
        55 => "_UltBookSmitePlaceholder".into(),
        _ => "Flash".into()
    };

    let summoner2: String = match map.get("summoner2Id").unwrap().as_i64().unwrap() {
        1 => "Boost".into(),
        3 => "Exhaust".into(),
        4 => "Flash".into(),
        6 => "Haste".into(),
        7 => "Heal".into(),
        11 => "Smite".into(),
        12 => "Teleport".into(),
        13 => "Mana".into(),
        14 => "Dot".into(),
        21 => "Barrier".into(),
        30 => "PoroRecall".into(),
        31 => "PoroThrow".into(),
        32 => "Snowball".into(),
        39 => "SnowURFSnowball_Mark".into(),
        54 => "_UltBookPlaceholder".into(),
        55 => "_UltBookSmitePlaceholder".into(),
        _ => "Flash".into()
    };

    let perks = map.get("perks").unwrap().as_object().unwrap().get("styles").unwrap().as_array().unwrap();

    let rune: String = match perks.get(0).unwrap().as_object().unwrap().get("selections").unwrap().as_array().unwrap().get(0).unwrap().as_object().unwrap().get("perk").unwrap().as_i64().unwrap() {
        8112 => "Electrocute".into(),
        8124 => "Predator".into(),
        8128 => "DarkHarvest".into(),
        9923 => "HailOfBlades".into(),

        8351 => "GlacialAugment".into(),
        8360 => "UnsealedSpellbook".into(),
        8369 => "FirstStrike".into(),

        8005 => "PressTheAttack".into(),
        8008 => "LethalTempo".into(),
        8010 => "Conqueror".into(),
        8021 => "FleetFootwork".into(),

        8437 => "GraspOfTheUndying".into(),
        8439 => "VeteranAftershock".into(),
        8465 => "Guardian".into(),

        8214 => "SummonAery".into(),
        8229 => "ArcaneComet".into(),
        8230 => "PhaseRush".into(),
        _ => "Electrocute".into()
    };

    let rune1: String = match perks.get(0).unwrap().as_object().unwrap().get("style").unwrap().as_i64().unwrap() {
        8000 => "Precision".into(),
        8100 => "Domination".into(),
        8200 => "Sorcery".into(),
        8300 => "Inspiration".into(),
        8400 => "Resolve".into(),
        _ => "Precision".into()
    };

    let rune2: String = match perks.get(1).unwrap().as_object().unwrap().get("style").unwrap().as_i64().unwrap() {
        8000 => "Precision".into(),
        8100 => "Domination".into(),
        8200 => "Sorcery".into(),
        8300 => "Inspiration".into(),
        8400 => "Resolve".into(),
        _ => "Precision".into()
    };

    Champion {

        win: map.get("win").unwrap().as_bool().unwrap(),
        champion_name: map.get("championName").unwrap().as_str().unwrap().into(),
        item0: map.get("item0").unwrap().as_i64().unwrap().into(),
        item1: map.get("item1").unwrap().as_i64().unwrap().into(),
        item2: map.get("item2").unwrap().as_i64().unwrap().into(),
        item3: map.get("item3").unwrap().as_i64().unwrap().into(),
        item4: map.get("item4").unwrap().as_i64().unwrap().into(),
        item5: map.get("item5").unwrap().as_i64().unwrap().into(),
        item6: map.get("item6").unwrap().as_i64().unwrap().into(),
        role: map.get("teamPosition").unwrap().as_str().unwrap().into(),
        rune,
        rune1,
        rune2,
        summoner1,
        summoner2,
        spellmax1,
        spellmax2,
        spellmax3,
        spellmax4,

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

    let response: serde_json::Value = ureq::get(&request)
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

fn read_matches(settings: &Settings, mut matches: HashSet<String>, requests: &mut u32, clock: &mut Instant) -> HashSet<Data> {
    let mut data: HashSet<Data> = HashSet::new();

    while matches.len() > 0 {

        if clock.elapsed().as_secs() > 120 {
            *clock = Instant::now();
            *requests = 80;
            println!("refresh");
        }

        if *requests == 0 {
            continue;
        }

        let element = matches.iter().next().cloned().unwrap();

        let target = read_match(settings, &element);
        if target.is_err() {
            *requests = 0;
            continue;
        }
        matches.take(&element).unwrap();

        data.insert(target.unwrap());

        *requests -= 1;
    }

    data
}

fn publish_data(db: &Database, data: HashSet<Data>) {
    let _ = db.add_data(data.into_iter().collect());
}

pub fn initialize_matches(settings: &mut Settings, db: Database) {
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
        filter_matches(&db, &mut matches);
        if matches.is_empty() {
            continue;
        }

        let data = read_matches(settings, matches, &mut requests, &mut clock);
        publish_data(&db, data);
    }
}
