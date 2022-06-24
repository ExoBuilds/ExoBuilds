use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Match {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub match_id: String,
}
