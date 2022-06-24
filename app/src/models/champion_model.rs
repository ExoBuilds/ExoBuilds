use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Champion {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub win: bool,
    pub champion_name: String,
    pub kills: u32,
    pub kda: f32,
    pub deaths: u32,
    pub item0: u32,
    pub item1: u32,
    pub item2: u32,
    pub item3: u32,
    pub item4: u32,
    pub item5: u32,
    pub item6: u32,
    pub role: String,
}
