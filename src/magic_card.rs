use serde_derive::*;

use serde::de::{self, Deserializer, Unexpected, Visitor};
use std::fmt;

#[derive(Debug, Deserialize)]
pub enum Color {
    #[serde(alias = "W")]
    White,
    #[serde(alias = "R")]
    Red,
    #[serde(alias = "U")]
    Blue,
    #[serde(alias = "B")]
    Black,
    #[serde(alias = "G")]
    Green,
}

#[derive(Debug, Deserialize)]
pub enum Supertype {
    Basic,
    Host,
    Legendary,
    Ongoing,
    Snow,
    World,
}

#[derive(Debug, Deserialize)]
pub enum Type {
    Artifact,
    Conspiracy,
    Creature,
    Enchantment,
    Hero,
    Instant,
    Land,
    Phenomenon,
    Plane,
    Planeswalker,
    Scheme,
    Sorcery,
    Summon,
    Tribal,
    Vanguard,
}

#[derive(Debug, Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    #[serde(alias = "Mythic")]
    MythicRare,
}

#[derive(Debug, Deserialize)]
pub enum Layout {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "split")]
    Split,
    #[serde(rename = "flip")]
    Flip,
    #[serde(rename = "doubleFaced")]
    DoubleFaced,
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "plane")]
    Plane,
    #[serde(rename = "scheme")]
    Scheme,
    #[serde(rename = "phenomenon")]
    Phenomenon,
    #[serde(rename = "leveler")]
    Leveler,
    #[serde(rename = "vanguard")]
    Vanguard,
    #[serde(rename = "aftermath")]
    Aftermath,
}

#[derive(Debug, Deserialize)]
pub struct Ruling {
    pub date: String,
    pub text: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForeignCard {
    pub name: String,
    pub text: String,
    pub flavor: Option<String>,
    pub image_url: String,
    pub language: String,
    #[serde(rename = "multiverseid")]
    pub multiverse_id: u32,
}

#[derive(Debug, Deserialize)]
pub enum Format {
    #[serde(rename = "1v1")]
    OneVsOne,
    Brawl,
    Commander,
    Duel,
    Frontier,
    Future,
    Legacy,
    Modern,
    Oldschool,
    Pauper,
    Penny,
    Standard,
    Vintage,
}

#[derive(Debug, Deserialize)]
pub enum Legality {
    Legal,
    Banned,
    Restricted,
}

#[derive(Debug, Deserialize)]
pub struct FormatLegality {
    pub format: Format,
    pub legality: Legality,
}

#[derive(Debug, Deserialize)]
pub enum BorderColor {
    #[serde(rename = "black")]
    Black,
    #[serde(rename = "borderless")]
    Borderless,
    #[serde(rename = "gold")]
    Gold,
    #[serde(rename = "silver")]
    Silver,
    #[serde(rename = "white")]
    White
}

fn string_as_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_f32(F32CMC)
}

struct F32CMC;

impl<'de> Visitor<'de> for F32CMC {
    type Value = f32;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representation of an f32")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        value.parse::<Self::Value>().map_err(|_err| {
            E::invalid_value(Unexpected::Str(value), &"a string representation of an f32")
        })
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> 
    where
        E: de::Error,
    {
        Ok(value as f32)
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E> 
    where
        E: de::Error,
    {
        Ok(value as f32)
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E> 
    where
        E: de::Error,
    {
        Ok(value as f32)
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> 
    where
        E: de::Error,
    {
        Ok(value as f32)
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> 
    where
        E: de::Error,
    {
        Ok(value as f32)
    }
}

#[derive(Debug, Deserialize)]
pub struct MagicCardResponse {
    pub card: Option<MagicCard>,
    pub cards: Option<Vec<MagicCard>>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MagicCard {
    pub name: String,
    pub names: Option<Vec<String>>,
    pub mana_cost: String,
    pub border_color: Option<BorderColor>,
    #[serde(deserialize_with = "string_as_f32")]
    pub cmc: f32,
    // Need to implement deserialize_with
    pub hand: Option<i32>,
    pub life: Option<i32>,
    pub loyalty: Option<String>,
    #[serde(rename = "isTimeshifted")]
    pub timeshifted: Option<bool>,
    pub colors: Vec<Color>,
    pub color_identity: Vec<Color>,
    #[serde(rename = "type")]
    pub card_type: String,
    pub supertypes: Vec<Supertype>,
    pub types: Vec<Type>,
    pub subtypes: Vec<String>,
    pub rarity: Rarity,
    pub set: String,
    pub set_name: String,
    pub text: String,
    pub flavor: Option<String>,
    pub artist: String,
    // Some cards have letters in their 'numbers' so this should be String
    pub number: String,
    pub power: Option<String>,
    pub toughness: Option<String>,
    pub layout: Layout,
    #[serde(rename = "multiverseid")]
    pub multiverse_id: u32,
    pub image_url: String,
    pub watermark: Option<String>,
    pub rulings: Vec<Ruling>,
    pub foreign_names: Vec<ForeignCard>,
    pub printings: Vec<String>,
    pub original_text: String,
    pub original_type: String,
    pub legalities: Vec<FormatLegality>,
    pub id: String,
}
