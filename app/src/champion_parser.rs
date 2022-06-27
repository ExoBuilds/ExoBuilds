use crate::{
    database::Database,
    models::{champion_model::Champion, recommended_champion_model::RecommendedChampion},
    utils::{get_champ_spells, get_champ_title},
};
use std::collections::HashMap;
use std::{thread, time};

fn get_highest_str(map: HashMap<String, i64>) -> String {
    let mut result = String::new();
    let mut tmp: i64 = -1;

    for (entry, value) in map {
        if tmp == -1 {
            result = entry;
            tmp = value;
            continue;
        }
        if value > tmp {
            result = entry;
            tmp = value;
        }
    }

    result
}

fn get_highest(map: HashMap<i64, i64>) -> i64 {
    let mut result: i64 = 0;
    let mut tmp: i64 = -1;

    for (entry, value) in map {
        if tmp == -1 {
            result = entry;
            tmp = value;
            continue;
        }
        if value > tmp {
            result = entry;
            tmp = value;
        }
    }

    result
}

fn get_next_item(item_id: u32, role: &String, matches: &Vec<Champion>) -> HashMap<i64, i64> {
    let mut item: HashMap<i64, i64> = HashMap::new();

    for element in matches {
        if &element.role != role {
            continue;
        }

        match item_id {
            0 => item.insert(
                element.item0,
                *(item.get(&element.item0).get_or_insert(&0)) + 1,
            ),
            1 => item.insert(
                element.item1,
                *(item.get(&element.item1).get_or_insert(&0)) + 1,
            ),
            2 => item.insert(
                element.item2,
                *(item.get(&element.item2).get_or_insert(&0)) + 1,
            ),
            3 => item.insert(
                element.item3,
                *(item.get(&element.item3).get_or_insert(&0)) + 1,
            ),
            4 => item.insert(
                element.item4,
                *(item.get(&element.item4).get_or_insert(&0)) + 1,
            ),
            5 => item.insert(
                element.item5,
                *(item.get(&element.item5).get_or_insert(&0)) + 1,
            ),
            _ => item.insert(
                element.item6,
                *(item.get(&element.item6).get_or_insert(&0)) + 1,
            ),
        };

    }

    item

}

fn get_next_rune(rune_id: u32, role: &String, matches: &Vec<Champion>) -> HashMap<String, i64> {
    let mut rune: HashMap<String, i64> = HashMap::new();

    for element in matches {
        if &element.role != role {
            continue;
        }

        match rune_id {
            0 => rune.insert(
                element.rune.clone(),
                *(rune.get(&element.rune).get_or_insert(&0)) + 1,
            ),
            1 => rune.insert(
                element.rune1.clone(),
                *(rune.get(&element.rune1).get_or_insert(&0)) + 1,
            ),
            _ => rune.insert(
                element.rune2.clone(),
                *(rune.get(&element.rune2).get_or_insert(&0)) + 1,
            ),
        };

    }

    rune

}

fn get_next_spell(spell_id: u32, role: &String, matches: &Vec<Champion>) -> HashMap<String, i64> {
    let mut spell: HashMap<String, i64> = HashMap::new();

    for element in matches {
        if &element.role != role {
            continue;
        }

        match spell_id {
            0 => spell.insert(
                element.spellmax1.clone(),
                *(spell.get(&element.spellmax1).get_or_insert(&0)) + 1,
            ),
            1 => spell.insert(
                element.spellmax2.clone(),
                *(spell.get(&element.spellmax2).get_or_insert(&0)) + 1,
            ),
            2 => spell.insert(
                element.spellmax3.clone(),
                *(spell.get(&element.spellmax3).get_or_insert(&0)) + 1,
            ),
            _ => spell.insert(
                element.spellmax4.clone(),
                *(spell.get(&element.spellmax4).get_or_insert(&0)) + 1,
            ),
        };

    }

    spell

}

fn get_next_summoner(summoner_id: u32, role: &String, matches: &Vec<Champion>) -> HashMap<String, i64> {
    let mut summoner: HashMap<String, i64> = HashMap::new();

    for element in matches {
        if &element.role != role {
            continue;
        }

        match summoner_id {
            0 => summoner.insert(
                element.summoner1.clone(),
                *(summoner.get(&element.summoner1).get_or_insert(&0)) + 1,
            ),
            _ => summoner.insert(
                element.summoner2.clone(),
                *(summoner.get(&element.summoner2).get_or_insert(&0)) + 1,
            ),
        };

    }

    summoner

}

fn remove_item_not_present(item_id: u32, item: i64, role: &String, matches: &mut Vec<Champion>) {

    match item_id {
        0 => matches.retain(|element| (&element.role == role && element.item0 == item)),
        1 => matches.retain(|element| (&element.role == role && element.item1 == item)),
        2 => matches.retain(|element| (&element.role == role && element.item2 == item)),
        3 => matches.retain(|element| (&element.role == role && element.item3 == item)),
        4 => matches.retain(|element| (&element.role == role && element.item4 == item)),
        5 => matches.retain(|element| (&element.role == role && element.item5 == item)),
        _ => matches.retain(|element| (&element.role == role && element.item6 == item)),
    };

}

fn remove_rune_not_present(rune_id: u32, rune: &String, role: &String, matches: &mut Vec<Champion>) {

    match rune_id {
        0 => matches.retain(|element| (&element.role == role && &element.rune == rune)),
        1 => matches.retain(|element| (&element.role == role && &element.rune1 == rune)),
        _ => matches.retain(|element| (&element.role == role && &element.rune2 == rune)),
    };

}

fn remove_spell_not_present(spell_id: u32, spell: &String, role: &String, matches: &mut Vec<Champion>) {

    match spell_id {
        0 => matches.retain(|element| (&element.role == role && &element.spellmax1 == spell)),
        1 => matches.retain(|element| (&element.role == role && &element.spellmax2 == spell)),
        2 => matches.retain(|element| (&element.role == role && &element.spellmax3 == spell)),
        _ => matches.retain(|element| (&element.role == role && &element.spellmax4 == spell)),
    };

}

fn read_matches(
    champion_name: String,
    matches: &Vec<Champion>,
    role: String,
) -> RecommendedChampion {
    let mut title: String = "".into();
    let mut wins = 0;
    let mut loses = 0;
    let item0: i64;
    let item1: i64;
    let item3: i64;
    let item2: i64;
    let item4: i64;
    let item5: i64;
    let item6: i64;
    let rune: String;
    let rune1: String;
    let rune2: String;
    let summoner1: String;
    let summoner2: String;
    let spellmax1: String;
    let spellmax2: String;
    let spellmax3: String;
    let spellmax4: String;
    let mut spellpath1: String = "ZoeQ".to_string();
    let mut spellpath2: String = "ZoeE".to_string();
    let mut spellpath3: String = "ZoeW".to_string();
    let mut spellpath4: String = "ZoeR".to_string();

    for element in matches {
        if element.role != role {
            continue;
        }

        match element.win {
            true => wins += 1,
            false => loses += 1,
        };
    }

    let mut tmp_matches = matches.clone();

    item0 = get_highest(get_next_item(0, &role, &tmp_matches));
    remove_item_not_present(0, item0, &role, &mut tmp_matches);
    item1 = get_highest(get_next_item(1, &role, &tmp_matches));
    remove_item_not_present(1, item1, &role, &mut tmp_matches);
    item2 = get_highest(get_next_item(2, &role, &tmp_matches));
    remove_item_not_present(2, item2, &role, &mut tmp_matches);
    item3 = get_highest(get_next_item(3, &role, &tmp_matches));
    remove_item_not_present(3, item3, &role, &mut tmp_matches);
    item4 = get_highest(get_next_item(4, &role, &tmp_matches));
    remove_item_not_present(4, item4, &role, &mut tmp_matches);
    item5 = get_highest(get_next_item(5, &role, &tmp_matches));
    remove_item_not_present(5, item5, &role, &mut tmp_matches);
    item6 = get_highest(get_next_item(6, &role, &tmp_matches));

    let mut tmp_matches = matches.clone();
    rune = get_highest_str(get_next_rune(0, &role, &tmp_matches));
    remove_rune_not_present(0, &rune, &role, &mut tmp_matches);
    rune1 = get_highest_str(get_next_rune(1, &role, &tmp_matches));
    remove_rune_not_present(1, &rune1, &role, &mut tmp_matches);
    rune2 = get_highest_str(get_next_rune(2, &role, &tmp_matches));

    let mut tmp_matches = matches.clone();
    spellmax1 = get_highest_str(get_next_spell(0, &role, &tmp_matches));
    remove_spell_not_present(0, &spellmax1, &role, &mut tmp_matches);
    spellmax2 = get_highest_str(get_next_spell(1, &role, &tmp_matches));
    remove_spell_not_present(1, &spellmax2, &role, &mut tmp_matches);
    spellmax3 = get_highest_str(get_next_spell(2, &role, &tmp_matches));
    remove_spell_not_present(2, &spellmax3, &role, &mut tmp_matches);
    spellmax4 = get_highest_str(get_next_spell(3, &role, &tmp_matches));

    let mut tmp_matches = matches.clone();
    summoner1 = get_highest_str(get_next_summoner(0, &role, &tmp_matches));
    tmp_matches.retain(|element| &element.role == &role && &element.summoner1 == &summoner1);
    summoner2 = get_highest_str(get_next_summoner(1, &role, &tmp_matches));

    let tmp_title = get_champ_title(&champion_name);

    if tmp_title.is_ok() {
        title = tmp_title.unwrap();
    }

    let spell_icons = get_champ_spells(&champion_name);

    if spell_icons.is_ok() {
        let spell_icons = spell_icons.unwrap();
        if spell_icons.contains_key(&spellmax1) {
            spellpath1 = spell_icons.get(&spellmax1).unwrap().to_string();
        }
        if spell_icons.contains_key(&spellmax2) {
            spellpath2 = spell_icons.get(&spellmax2).unwrap().to_string();
        }
        if spell_icons.contains_key(&spellmax3) {
            spellpath3 = spell_icons.get(&spellmax3).unwrap().to_string();
        }
        if spell_icons.contains_key(&spellmax4) {
            spellpath4 = spell_icons.get(&spellmax4).unwrap().to_string();
        }
    }

    RecommendedChampion {
        id: None,
        wins,
        loses,
        name: champion_name,
        title,
        item0,
        item1,
        item2,
        item3,
        item4,
        item5,
        item6,
        role,
        rune,
        rune1,
        rune2,
        summoner1,
        summoner2,
        spellmax1,
        spellmax2,
        spellmax3,
        spellmax4,
        spellpath1,
        spellpath2,
        spellpath3,
        spellpath4,
    }
}

fn update_champion(db: &Database, champion_name: String, matches: Vec<Champion>) {
    let _ =
        db.update_recommended_champion(read_matches(champion_name.clone(), &matches, "TOP".into()));
    let _ = db.update_recommended_champion(read_matches(
        champion_name.clone(),
        &matches,
        "JUNGLE".into(),
    ));
    let _ = db.update_recommended_champion(read_matches(
        champion_name.clone(),
        &matches,
        "MIDDLE".into(),
    ));
    let _ =
        db.update_recommended_champion(read_matches(champion_name.clone(), &matches, "BOTTOM".into()));
    let _ = db.update_recommended_champion(read_matches(
        champion_name.clone(),
        &matches,
        "UTILITY".into(),
    ));
}

pub fn initialize_champions(db: Database) {
    loop {
        let data = db.get_champions();

        if data.is_err() {
            continue;
        }

        let data = data.unwrap();

        for (champion_name, matches) in data {
            update_champion(&db, champion_name, matches);
        }

        // update every 15 mins
        thread::sleep(time::Duration::from_secs(15*60));
    }
}
