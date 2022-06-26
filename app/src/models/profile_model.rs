use serde::{Deserialize, Serialize};

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct Profile {
    pub name: String,
    pub puuid: String,
}
