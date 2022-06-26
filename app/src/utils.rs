use std::collections::HashMap;

use crate::models::data_model::Data;

// (champion_name: win, loses) 
fn get_highest(map: &HashMap<String, (i64, i64)>) -> (String, i64, i64) {

    let mut result: (String, i64, i64) = ("".into(), -1, -1);

    for (entry, value) in map {
        if result.1 == -1 {
            result.0 = entry.clone();
            result.1 = value.0;
            result.2 = value.1;
            continue;
        }
        if value.0 + value.1 > result.1 + result.2 {
            result.0 = entry.clone();
            result.1 = value.0;
            result.2 = value.1;
        }
    }

    result
}

pub fn get_most_played_champs(puuid: String, data: &Vec<Data>) -> Vec<(String, String, String)> {

    // (champion_name, number of samples, winrate)
    let mut champions: Vec<(String, String, String)> = Vec::new();

    // (champion_name: wins, loses)
    let mut tmp: HashMap<String, (i64, i64)> = HashMap::new();

    for element in data {
        for t in &element.champions {
            if t.puuid == puuid {
                let value: &(i64, i64) = tmp.get(&t.champion_name).get_or_insert(&(0, 0));
                let mut result: (i64, i64) = *value;
                if t.win {
                    result.0 = result.0 + 1;
                } else {
                    result.1 = result.1 + 1;
                }
                tmp.insert(t.champion_name.clone(), result);
            }
        }
    }

    for _ in 0..3 {
        let highest = get_highest(&tmp);
        champions.push((highest.0.clone(), highest.1.to_string(), highest.2.to_string()));
        tmp.remove(&highest.0);
    }

    champions

}

pub fn get_latest_icon(puuid: String, data: &Vec<Data>) -> String {

    let mut icon: String = "4603".into();

    if data.len() > 0 {
        for champ in &(data.get(0).unwrap().champions) {

            if champ.puuid == puuid {
                icon = champ.profile_icon.to_string();
            }

        }
    }

    icon

}
