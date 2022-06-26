use crate::database::Database;
use crate::matches_parser::serde_json::Map;
use crate::matches_parser::serde_json::Value;
use crate::models::champion_model::Champion;
use crate::models::data_model::Data;
use crate::settings::Settings;
use ureq::serde_json;

use std::collections::HashSet;

fn retrieve_match(settings: &Settings, puuid: &String, queue_type: u16, count: usize) -> Result<HashSet<String>, ureq::Error> {
    let mut matches: HashSet<String> = HashSet::new();

    let queue = match queue_type {
        1 => "ranked",
        _ => "normal"
    };

    let request = format!(
        "https://europe.api.riotgames.com/lol/match/v5/matches/by-puuid/{puuid}/ids?type={queue}&start=0&count={ids}",
        puuid = puuid,
        queue = queue,
        ids = count
    );

    let mut response: serde_json::Value = ureq::get(&request)
        .set("X-Riot-Token", &settings.riot_api)
        .call()?
        .into_json()?;
    Ok({
        if response.as_array_mut().is_some() {
            let response = response.as_array_mut().unwrap();

            for element in response.to_vec().into_iter() {
                matches.insert(element.as_str().unwrap().into());
            }
        }

        matches
    })
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
        _ => "".into(),
    };

    spells.remove(0);

    let spellmax2: String = match spells.get(0).unwrap() {
        target if target == &q => "Q".into(),
        target if target == &w => "W".into(),
        target if target == &e => "E".into(),
        target if target == &r => "R".into(),
        _ => "".into(),
    };

    spells.remove(0);

    let spellmax3: String = match spells.get(0).unwrap() {
        target if target == &q => "Q".into(),
        target if target == &w => "W".into(),
        target if target == &e => "E".into(),
        target if target == &r => "R".into(),
        _ => "".into(),
    };

    spells.remove(0);

    let spellmax4: String = match spells.get(0).unwrap() {
        target if target == &q => "Q".into(),
        target if target == &w => "W".into(),
        target if target == &e => "E".into(),
        target if target == &r => "R".into(),
        _ => "".into(),
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
        _ => "Flash".into(),
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
        _ => "Flash".into(),
    };

    let perks = map
        .get("perks")
        .unwrap()
        .as_object()
        .unwrap()
        .get("styles")
        .unwrap()
        .as_array()
        .unwrap();

    let rune: String = match perks
        .get(0)
        .unwrap()
        .as_object()
        .unwrap()
        .get("selections")
        .unwrap()
        .as_array()
        .unwrap()
        .get(0)
        .unwrap()
        .as_object()
        .unwrap()
        .get("perk")
        .unwrap()
        .as_i64()
        .unwrap()
    {
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
        _ => "Electrocute".into(),
    };

    let rune1: String = match perks
        .get(0)
        .unwrap()
        .as_object()
        .unwrap()
        .get("style")
        .unwrap()
        .as_i64()
        .unwrap()
    {
        8000 => "Precision".into(),
        8100 => "Domination".into(),
        8200 => "Sorcery".into(),
        8300 => "Inspiration".into(),
        8400 => "Resolve".into(),
        _ => "Precision".into(),
    };

    let rune2: String = match perks
        .get(1)
        .unwrap()
        .as_object()
        .unwrap()
        .get("style")
        .unwrap()
        .as_i64()
        .unwrap()
    {
        8000 => "Precision".into(),
        8100 => "Domination".into(),
        8200 => "Sorcery".into(),
        8300 => "Inspiration".into(),
        8400 => "Resolve".into(),
        _ => "Precision".into(),
    };

    Champion {
        win: map.get("win").unwrap().as_bool().unwrap(),
        kills: map.get("kills").unwrap().as_i64().unwrap(),
        deaths: map.get("deaths").unwrap().as_i64().unwrap(),
        assists: map.get("assists").unwrap().as_i64().unwrap(),
        champion_name: map.get("championName").unwrap().as_str().unwrap().into(),
        minions_killed: map.get("totalMinionsKilled").unwrap().as_i64().unwrap(),
        neutral_minions_killed: map.get("neutralMinionsKilled").unwrap().as_i64().unwrap(),
        total_minions_killed: map.get("totalMinionsKilled").unwrap().as_i64().unwrap() + map.get("neutralMinionsKilled").unwrap().as_i64().unwrap(),
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
        puuid: map.get("puuid").unwrap().as_str().unwrap().into(),
        profile_icon: map.get("profileIcon").unwrap().as_i64().unwrap().into(),
    }
}

fn read_match(settings: &Settings, target: &String) -> Result<Data, ureq::Error> {
    let mut data = Data {
        id: None,
        match_duration: 0,
        match_id: target.into(),
        champions: Vec::new(),
        match_creation: 0,
    };

    let request = format!(
        "https://europe.api.riotgames.com/lol/match/v5/matches/{id}",
        id = target
    );

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

                data.match_duration = element.get("gameDuration").unwrap().as_i64().unwrap();
                data.match_creation = element.get("gameCreation").unwrap().as_i64().unwrap();

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

fn read_matches(
    db: &Database,
    settings: &Settings,
    mut matches: HashSet<String>,
) {
    while matches.len() > 0 {

        let element = matches.iter().next().cloned().unwrap();

        let target = read_match(settings, &element);
        if target.is_err() {
            continue;
        }
        matches.take(&element).unwrap();

        let _ = db.add_data(target.unwrap());
    }
}

pub fn update_latest_matches(settings: &Settings, db: &Database, puuid: &String) {
    let matches = retrieve_match(settings, puuid, 0, 5);

    if matches.is_err() {
        return;
    }

    let matches2 = retrieve_match(settings, puuid, 1, 5);

    if matches2.is_err() {
        return;
    }

    let mut matches = matches.unwrap();

    matches.extend(matches2.unwrap());

    if matches.is_empty() {
        return;
    }
    filter_matches(&db, &mut matches);
    if matches.is_empty() {
        return;
    }

    read_matches(db, settings, matches);
}
