use serde::{Deserialize, Serialize};

use super::{champion_model::Champion, data_model::Data};

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize)]
pub struct MatchHistory {
    pub player_champion: Champion,
    pub data: Data,
}
