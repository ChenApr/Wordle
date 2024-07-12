use clap::builder::Str;
use config::{Config, ConfigError, File, FileFormat};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct GameConfig {
    
    pub random: Option<bool>,
    pub difficult: Option<bool>,
    pub stats: Option<bool>,
    pub day: Option<u64>,
    pub seed: Option<u64>,
    pub final_set: Option<String>,
    pub acceptable_set: Option<String>,
    pub state: Option<String>,
    pub word: Option<String>
}

//Deposed later. Write here due to the misuse of Config::builder()
impl GameConfig {
    pub fn new() -> GameConfig {
        GameConfig {
            random: None,
            difficult: None,
            stats: None,
            day: None,
            seed: None,
            final_set: None,
            acceptable_set: None,
            state: None,
            word: None
        }
    }
    pub fn new_with_day(day: u64) -> GameConfig {
        GameConfig {
            random: None,
            difficult: None,
            stats: None,
            day: Some(day),
            seed: None,
            final_set: None,
            acceptable_set: None,
            state: None,
            word: None
        }
    }


}

//load config to GameConfig
pub fn load_config(c: &String) -> Result<GameConfig, ConfigError> {

    let mut _config = Config::builder().add_source(File::new(&c, FileFormat::Json)).build()?;

    _config.try_deserialize()

}

