use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Datacron {}

#[derive(Debug, Serialize, Deserialize)]
pub struct SWgohMod {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    #[serde(rename = "data")]
    pub datas: Data,
    //#[serde(rename = "units")]
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
    ship_battles_won: i32,
    pvp_battles_won: i32,
    pve_battles_won: i32,
    pve_hard_won: i32,
    galactic_war_won: i32,
    guild_raid_won: i32,
    guild_contribution: i32,
    guild_exchange_donations: i32,
    season_full_clears: i32,
    season_successful_defends: i32,
    season_league_score: i32,
    season_undersized_squad_wins: i32,
    season_promotions_earned: i32,
    season_banners_earned: i32,
    season_offensive_battles_won: i32,
    season_territories_defeated: i32,
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
    combat_type: i8, // 1 char, 2 ship
    relic_tier: i8,  // should be minus 2 for real value (r0 = 1, r1 = 2 ... )
    stats: Stats,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    #[serde(rename = "1")]
    health: f32,
    #[serde(rename = "2")]
    strength: f32,
    #[serde(rename = "3")]
    agility: f32,
    #[serde(rename = "4")]
    tactics: f32,
    #[serde(rename = "5")]
    speed: f32,
    #[serde(rename = "6")]
    physical_damage: f32,
    #[serde(rename = "7")]
    special_damage: f32,
    #[serde(rename = "8")]
    armor: f32,
    #[serde(rename = "9")]
    resistance: f32,
    #[serde(rename = "10")]
    armor_penetration: f32,
    #[serde(rename = "11")]
    resistance_penetration: f32,
    #[serde(rename = "12")]
    dodge_chance: f32,
    #[serde(rename = "13")]
    deflection_chance: f32,
}
