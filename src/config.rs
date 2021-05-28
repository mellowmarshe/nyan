use std::{
    collections::{HashMap, HashSet},
    fs,
    path::PathBuf,
};

use serde::{Deserialize, Serializer};
use serenity::{model::id::UserId, utils::Color};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub bot: BotConfig,
    pub areas: HashMap<String, AreasConfig>,
    pub database: DatabaseConfig,
    pub sentry: SentryConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BotConfig {
    pub token: String,
    pub prefix: String,
    pub version: String,
    #[serde(serialize_with = "serialize_color")]
    pub color: Color,
    #[serde(serialize_with = "serialize_owners")]
    pub owners: HashSet<UserId>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AreasConfig {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub postgres: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SentryConfig {
    pub url: String,
    pub environment: String,
}

#[allow(dead_code)]
fn serialize_color<S>(color: &u32, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let color = Color::new(*color);
    s.serialize_some(&color)
}

#[allow(dead_code)]
fn serialize_owners<S>(owners: &[i64], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let owners: HashSet<UserId> = owners.iter().map(|i| UserId(*i as u64)).collect();
    s.serialize_some(&owners)
}

impl Config {
    pub fn new(path: PathBuf) -> Self {
        let content = fs::read_to_string(path).unwrap();
        let config: Config = toml::from_str(&content).unwrap();

        config
    }
}
