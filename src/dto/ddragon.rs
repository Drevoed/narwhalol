use std::collections::HashMap;

#[derive(Debug)]
pub struct DDragonResponse;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AllChampions
{
    #[serde(rename = "type")]
    pub data_type: String,
    pub format: String,
    pub version: String,
    pub data: HashMap<String, ChampionData>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChampionData {
    pub version: String,
    pub id: String,
    pub key: String,
    pub name: String,
    pub title: String,
    pub blurb: String,
    pub info: ChampionDataInfo,
    pub image: ChampionDataImage,
    pub tags: Vec<String>,
    pub partype: String,
    pub stats: ChampionDataStats,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChampionDataInfo {
    pub attack: i32,
    pub defense: i32,
    pub magic: i32,
    pub difficulty: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChampionDataImage {
    pub full: String,
    pub sprite: String,
    pub group: String,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChampionDataStats {
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChampionDataFull {
    #[serde(rename = "type")]
    pub data_type: String,
    pub format: String,
    pub version: String,
    pub data: HashMap<String, ExtendedData>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ExtendedData {
    pub skins: Vec<SkinData>,
    pub lore: String,
    pub allytips: Vec<String>,
    pub enemytips: Vec<String>,
    pub spells: Vec<SpellData>,
    pub passive: PassiveData,
    pub recommendeditems: Vec<RecommendedItemData>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SkinData {
    pub id: String,
    pub num: i32,
    pub name: String,
    pub chromas: bool
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SpellData {
    pub id: String,
    pub name: String,
    #[serde(rename = "abstract")]
    pub description: String,
    pub tooltip: String,
    pub leveltip: SpellLevelTip,
    pub maxrank: i32,
    pub cooldown: [f64; 4],
    pub 
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SpellLevelTip {
    pub label: Vec<String>,
    pub effect: Vec<String>
}


