use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Datacron {}

#[derive(Debug, Serialize, Deserialize)]
pub struct SWgohMod {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    #[serde(rename = "data")]
    pub datas: Data,
    #[serde(rename = "units")]
    pub units: Vec<Unit>,
    //pub mods: Vec<swgohmod>,
    //pub datacrons: Vec<Datacron>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    ally_code: i32,
    arena_leader_base_id: String,
    arena_rank: i16,
    level: i8,
    name: String,
    galactic_power: i32,
    character_galactic_power: i32,
    ship_galactic_power: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Unit {
    #[serde(rename = "data")]
    unit_data: UnitData,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UnitData {
    base_id: String,
    name: String,
    gear_level: i8,
    level: i8,
    power: i32,
    rarity: i8,
}
