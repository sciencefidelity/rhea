#![allow(unused)]
use std::fs;

use anyhow::Result;
use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub user: User,
    pub package: Package,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub name: String,
    pub email: String,
    pub github_username: String,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub license: String,
}

const CONFIG_PATH: &str = "config.toml";

pub fn read() -> Result<Config> {
    let filename = CONFIG_PATH;
    let contents = fs::read_to_string(filename)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}
