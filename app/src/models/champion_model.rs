use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize)]
pub struct Champion {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub win: bool,
    pub champion_name: String,
    pub kills: i64,
    pub assists: i64,
    pub deaths: i64,
    pub item0: i64,
    pub item1: i64,
    pub item2: i64,
    pub item3: i64,
    pub item4: i64,
    pub item5: i64,
    pub item6: i64,
    pub role: String,
}
