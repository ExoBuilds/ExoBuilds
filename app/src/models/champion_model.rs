use serde::{Serialize, Deserialize};

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize)]
pub struct Champion {
    pub win: bool,
    pub champion_name: String,
    pub item0: i64,
    pub item1: i64,
    pub item2: i64,
    pub item3: i64,
    pub item4: i64,
    pub item5: i64,
    pub item6: i64,
    pub role: String,
    pub rune: String,
    pub rune1: String,
    pub rune2: String,
    pub summoner1: String,
    pub summoner2: String,
    pub spellmax1: String,
    pub spellmax2: String,
    pub spellmax3: String,
    pub spellmax4: String,
    pub puuid: String,
    pub profile_icon: i64,
}
