use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

use super::champion_model::Champion;

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub match_id: String,
    pub match_duration: i64,
    pub match_creation: i64,
    pub champions: Vec<Champion>,
}
