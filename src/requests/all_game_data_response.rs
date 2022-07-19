use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AllGameDataResponse {
  #[serde(rename = "activePlayer")]
  pub active_player: ActivePlayer,

  #[serde(rename = "allPlayers")]
  pub all_players: Vec<Player>,

  #[serde(rename = "events")]
  pub events: Events,

  #[serde(rename = "gameData")]
  pub game_data: GameData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivePlayer {
  #[serde(rename = "abilities")]
  pub abilities: Abilities,

  #[serde(rename = "championStats")]
  pub champion_stats: ChampionStats,

  #[serde(rename = "currentGold")]
  pub current_gold: f64,

  #[serde(rename = "fullRunes")]
  pub full_runes: FullRunes,

  #[serde(rename = "level")]
  pub level: i64,

  #[serde(rename = "summonerName")]
  pub summoner_name: String,

  #[serde(rename = "teamRelativeColors")]
  pub team_relative_colors: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Abilities {
  #[serde(rename = "Passive")]
  pub passive: Ability,

  #[serde(rename = "Q")]
  pub q: Ability,

  #[serde(rename = "W")]
  pub w: Ability,

  #[serde(rename = "E")]
  pub e: Ability,

  #[serde(rename = "R")]
  pub r: Ability,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ability {
  #[serde(rename = "abilityLevel")]
  pub ability_level: Option<i64>,

  #[serde(rename = "displayName")]
  pub display_name: String,

  #[serde(rename = "id")]
  pub id: Option<String>,

  #[serde(rename = "rawDescription")]
  pub raw_description: String,

  #[serde(rename = "rawDisplayName")]
  pub raw_display_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChampionStats {
  #[serde(rename = "abilityHaste")]
  pub ability_haste: f64,

  #[serde(rename = "abilityPower")]
  pub ability_power: f64,

  #[serde(rename = "armor")]
  pub armor: f64,

  #[serde(rename = "armorPenetrationFlat")]
  pub armor_penetration_flat: f64,

  #[serde(rename = "armorPenetrationPercent")]
  pub armor_penetration_percent: f64,

  #[serde(rename = "attackDamage")]
  pub attack_damage: f64,

  #[serde(rename = "attackRange")]
  pub attack_range: f64,

  #[serde(rename = "attackSpeed")]
  pub attack_speed: f64,

  #[serde(rename = "bonusArmorPenetrationPercent")]
  pub bonus_armor_penetration_percent: f64,

  #[serde(rename = "bonusMagicPenetrationPercent")]
  pub bonus_magic_penetration_percent: f64,

  #[serde(rename = "critChance")]
  pub crit_chance: f64,

  #[serde(rename = "critDamage")]
  pub crit_damage: f64,

  #[serde(rename = "currentHealth")]
  pub current_health: f64,

  #[serde(rename = "healShieldPower")]
  pub heal_shield_power: f64,

  #[serde(rename = "healthRegenRate")]
  pub health_regen_rate: f64,

  #[serde(rename = "lifeSteal")]
  pub life_steal: f64,

  #[serde(rename = "magicLethality")]
  pub magic_lethality: f64,

  #[serde(rename = "magicPenetrationFlat")]
  pub magic_penetration_flat: f64,

  #[serde(rename = "magicPenetrationPercent")]
  pub magic_penetration_percent: f64,

  #[serde(rename = "magicResist")]
  pub magic_resist: f64,

  #[serde(rename = "maxHealth")]
  pub max_health: f64,

  #[serde(rename = "moveSpeed")]
  pub move_speed: f64,

  #[serde(rename = "omnivamp")]
  pub omnivamp: f64,

  #[serde(rename = "physicalLethality")]
  pub physical_lethality: f64,

  #[serde(rename = "physicalVamp")]
  pub physical_vamp: f64,

  #[serde(rename = "resourceMax")]
  pub resource_max: f64,

  #[serde(rename = "resourceRegenRate")]
  pub resource_regen_rate: f64,

  #[serde(rename = "resourceType")]
  pub resource_type: String,

  #[serde(rename = "resourceValue")]
  pub resource_value: f64,

  #[serde(rename = "spellVamp")]
  pub spell_vamp: f64,

  #[serde(rename = "tenacity")]
  pub tenacity: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FullRunes {
  #[serde(rename = "generalRunes")]
  pub general_runes: Vec<Keystone>,

  #[serde(rename = "keystone")]
  pub keystone: Keystone,

  #[serde(rename = "primaryRuneTree")]
  pub primary_rune_tree: Keystone,

  #[serde(rename = "secondaryRuneTree")]
  pub secondary_rune_tree: Keystone,

  #[serde(rename = "statRunes")]
  pub stat_runes: Vec<StatRune>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Keystone {
  #[serde(rename = "displayName")]
  pub display_name: String,

  #[serde(rename = "id")]
  pub id: i64,

  #[serde(rename = "rawDescription")]
  pub raw_description: String,

  #[serde(rename = "rawDisplayName")]
  pub raw_display_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatRune {
  #[serde(rename = "id")]
  pub id: i64,

  #[serde(rename = "rawDescription")]
  pub raw_description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
  #[serde(rename = "championName")]
  pub champion_name: String,

  #[serde(rename = "isBot")]
  pub is_bot: bool,

  #[serde(rename = "isDead")]
  pub is_dead: bool,

  #[serde(rename = "items")]
  pub items: Vec<Item>,

  #[serde(rename = "level")]
  pub level: i64,

  #[serde(rename = "position")]
  pub position: String,

  #[serde(rename = "rawChampionName")]
  pub raw_champion_name: String,

  #[serde(rename = "respawnTimer")]
  pub respawn_timer: f64,

  #[serde(rename = "runes")]
  pub runes: Runes,

  #[serde(rename = "scores")]
  pub scores: Scores,

  #[serde(rename = "skinID")]
  pub skin_id: i64,

  #[serde(rename = "summonerName")]
  pub summoner_name: String,

  #[serde(rename = "summonerSpells")]
  pub summoner_spells: SummonerSpells,

  #[serde(rename = "team")]
  pub team: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
  #[serde(rename = "canUse")]
  pub can_use: bool,

  #[serde(rename = "consumable")]
  pub consumable: bool,

  #[serde(rename = "count")]
  pub count: i64,

  #[serde(rename = "displayName")]
  pub display_name: String,

  #[serde(rename = "itemID")]
  pub item_id: i64,

  #[serde(rename = "price")]
  pub price: i64,

  #[serde(rename = "rawDescription")]
  pub raw_description: String,

  #[serde(rename = "rawDisplayName")]
  pub raw_display_name: String,

  #[serde(rename = "slot")]
  pub slot: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Runes {
  #[serde(rename = "keystone")]
  pub keystone: Keystone,

  #[serde(rename = "primaryRuneTree")]
  pub primary_rune_tree: Keystone,

  #[serde(rename = "secondaryRuneTree")]
  pub secondary_rune_tree: Keystone,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Scores {
  #[serde(rename = "assists")]
  pub assists: i64,

  #[serde(rename = "creepScore")]
  pub creep_score: i64,

  #[serde(rename = "deaths")]
  pub deaths: i64,

  #[serde(rename = "kills")]
  pub kills: i64,

  #[serde(rename = "wardScore")]
  pub ward_score: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SummonerSpells {
  #[serde(rename = "summonerSpellOne")]
  pub summoner_spell_one: Ability,

  #[serde(rename = "summonerSpellTwo")]
  pub summoner_spell_two: Ability,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Events {
  #[serde(rename = "Events")]
  pub events: Vec<Event>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
  #[serde(rename = "EventID")]
  pub event_id: i64,

  #[serde(rename = "EventName")]
  pub event_name: String,

  #[serde(rename = "EventTime")]
  pub event_time: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameData {
  #[serde(rename = "gameMode")]
  pub game_mode: String,

  #[serde(rename = "gameTime")]
  pub game_time: f64,

  #[serde(rename = "mapName")]
  pub map_name: String,

  #[serde(rename = "mapNumber")]
  pub map_number: i64,

  #[serde(rename = "mapTerrain")]
  pub map_terrain: String,
}
