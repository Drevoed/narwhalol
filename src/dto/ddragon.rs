use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct AllChampions {
    #[serde(rename = "type")]
    pub data_type: String,
    pub format: String,
    pub version: String,
    pub data: HashMap<String, ChampionData>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChampionExtended {
    #[serde(rename = "type")]
    pub type_field: String,
    pub format: String,
    pub version: String,
    pub data: HashMap<String, ChampionFullData>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChampionData {
    pub version: String,
    pub id: String,
    #[serde(with = "string")]
    pub key: u64,
    pub name: String,
    pub title: String,
    pub blurb: String,
    pub info: ChampionInfoData,
    pub image: ChampionImageData,
    pub tags: Vec<String>,
    pub partype: String,
    pub stats: ChampionStatsData,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChampionInfoData {
    pub attack: i32,
    pub defense: i32,
    pub magic: i32,
    pub difficulty: i32,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChampionImageData {
    pub full: String,
    pub sprite: String,
    pub group: String,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChampionStatsData {
    pub hp: f64,
    pub hpperlevel: f64,
    pub mp: f64,
    pub mpperlevel: f64,
    pub movespeed: f64,
    pub armor: f64,
    pub armorperlevel: f64,
    pub spellblock: f64,
    pub spellblockperlevel: f64,
    pub attackrange: f64,
    pub hpregen: f64,
    pub hpregenperlevel: f64,
    pub mpregen: f64,
    pub mpregenperlevel: f64,
    pub crit: f64,
    pub critperlevel: f64,
    pub attackdamage: f64,
    pub attackdamageperlevel: f64,
    pub attackspeedperlevel: f64,
    pub attackspeed: f64,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChampionFullData {
    pub id: String,
    #[serde(with = "string")]
    pub key: u64,
    pub name: String,
    pub title: String,
    pub image: ChampionImageData,
    pub skins: Vec<ChampionSkinData>,
    pub lore: String,
    pub blurb: String,
    pub allytips: Vec<String>,
    pub enemytips: Vec<String>,
    pub tags: Vec<String>,
    pub partype: String,
    pub info: ChampionInfoData,
    pub stats: ChampionStatsData,
    pub spells: Vec<ChampionSpellData>,
    pub passive: ChampionPassiveData,
    pub recommended: Vec<ChampionRecommendedData>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChampionSkinData {
    pub id: String,
    pub num: i32,
    pub name: String,
    pub chromas: bool,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChampionSpellData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tooltip: String,
    pub leveltip: ChampionSpellLevelTipData,
    pub maxrank: i32,
    pub cooldown: Vec<f64>,
    #[serde(rename = "cooldownBurn")]
    pub cooldown_burn: String,
    pub cost: Vec<f64>,
    #[serde(rename = "costBurn")]
    pub cost_burn: String,
    pub datavalues: ChampionDataValues,
    pub effect: Vec<Option<Vec<f64>>>,
    #[serde(rename = "effectBurn")]
    pub effect_burn: Vec<Option<String>>,
    pub vars: Vec<serde_json::Value>,
    #[serde(rename = "costType")]
    pub cost_type: String,
    pub maxammo: String,
    pub range: Vec<i64>,
    #[serde(rename = "rangeBurn")]
    pub range_burn: String,
    pub image: ChampionImageData,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChampionSpellLevelTipData {
    pub label: Vec<String>,
    pub effect: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChampionPassiveData {
    pub name: String,
    pub description: String,
    pub image: ChampionImageData,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChampionRecommendedData {
    pub champion: String,
    pub title: String,
    pub map: String,
    pub mode: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "customTag")]
    pub custom_tag: Option<String>,
    pub sortrank: Option<i64>,
    #[serde(rename = "extensionPage")]
    pub extension_page: Option<bool>,
    #[serde(rename = "customPanel")]
    pub custom_panel: Option<serde_json::Value>,
    pub blocks: Vec<ChampionBlockData>,
    #[serde(rename = "requiredPerk")]
    pub required_perk: Option<String>,
    #[serde(rename = "useObviousCheckmark")]
    pub use_obvious_checkmark: Option<bool>,
    pub priority: Option<bool>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChampionDataValues {}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChampionBlockData {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "recMath")]
    pub rec_math: Option<bool>,
    #[serde(rename = "recSteps")]
    pub rec_steps: Option<bool>,
    #[serde(rename = "minSummonerLevel")]
    pub min_summoner_level: Option<i64>,
    #[serde(rename = "maxSummonerLevel")]
    pub max_summoner_level: Option<i64>,
    #[serde(rename = "showIfSummonerSpell")]
    pub show_if_summoner_spell: Option<String>,
    #[serde(rename = "hideIfSummonerSpell")]
    pub hide_if_summoner_spell: Option<String>,
    #[serde(rename = "appendAfterSection")]
    pub append_after_section: Option<String>,
    #[serde(rename = "visibleWithAllOf")]
    #[serde(default)]
    pub visible_with_all_of: Vec<String>,
    #[serde(rename = "hiddenWithAnyOf")]
    #[serde(default)]
    pub hidden_with_any_of: Vec<String>,
    pub items: Vec<ChampionItemData>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChampionItemData {
    pub id: String,
    pub count: i64,
    #[serde(rename = "hideCount")]
    pub hide_count: Option<bool>,
}

mod string {
    use std::fmt::Display;
    use std::str::FromStr;

    use serde::{de, Serializer, Deserialize, Deserializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
        where T: Display,
              S: Serializer
        {
            serializer.collect_str(value)
        }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
        where T: FromStr,
              T::Err: Display,
              D: Deserializer<'de>
        {
            String::deserialize(deserializer)?.parse().map_err(de::Error::custom)
        }
}
