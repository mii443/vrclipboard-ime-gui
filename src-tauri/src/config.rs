use std::{fs::File, io::{Read, Write}, path::{Path, PathBuf}};

use platform_dirs::AppDirs;
use serde::Serialize;
use serde_derive::Deserialize;
use anyhow::Result;
use tauri::State;

use crate::AppState;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(default = "semicolon" )]
    pub prefix: String,
    #[serde(default = "slash" )]
    pub split: String,
    #[serde(default = "semicolon" )]
    pub command: String,
    #[serde(default = "bool_true")]
    pub ignore_prefix: bool,
    #[serde(default)]
    pub on_copy_mode: OnCopyMode,
    #[serde(default = "bool_true")]
    pub skip_url: bool
}

impl Default for Config {
    fn default() -> Self {
        Self { 
            prefix: ";".to_string(), 
            split: "/".to_string(), 
            command: ";".to_string(), 
            ignore_prefix: true, 
            on_copy_mode: OnCopyMode::ReturnToChatbox ,
            skip_url: true
        }
    }
}

#[inline]
fn slash() -> String { String::from("/") }
#[inline]
fn semicolon() -> String { String::from(";") }
#[inline]
fn bool_true() -> bool { true }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum OnCopyMode {
    ReturnToClipboard,
    ReturnToChatbox,
    SendDirectly
}

impl Default for OnCopyMode {
    fn default() -> Self {
        Self::ReturnToChatbox
    }
}

impl Config {
    pub fn load() -> Result<Config> {
        std::fs::create_dir_all(Self::get_path()).unwrap();

        if !Path::new(&Self::get_path().join("config.yaml")).exists() {
            Self::generate_default_config()?;
        }
        let mut file = File::open(&Self::get_path().join("config.yaml"))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }

    pub fn save(&self, state: State<AppState>) -> Result<(), String> {
        std::fs::create_dir_all(Self::get_path()).unwrap();

        let mut file = match File::create(&Self::get_path().join("config.yaml")) {
            Ok(file) => file,
            Err(e) => {
                println!("Err: {:?}", e);
                return Err(format!("Failed to create config file: {}", e))
            },
        };
    
        match serde_yaml::to_string(&self) {
            Ok(yaml) => {
                if let Err(e) = file.write_all(yaml.as_bytes()) {
                    println!("Err: {:?}", e);
                    return Err(format!("Failed to write config: {}", e));
                }
                let mut app_config = state.config.lock().unwrap();
                *app_config = self.clone();
                Ok(())
            },
            Err(e) => {
                println!("Err: {:?}", e);
                Err(format!("Failed to serialize config: {}", e))
            },
        }
    }

    pub fn generate_default_config() -> Result<()> {
        let mut file = File::create(&Self::get_path().join("config.yaml"))?;
        file.write_all(serde_yaml::to_string(&Config::default()).unwrap().as_bytes())?;
        file.flush()?;
        Ok(())
    }

    pub fn get_path() -> PathBuf {
        let app_dirs = AppDirs::new(Some("vrclipboard-ime"), false).unwrap();
        let app_data = app_dirs.config_dir;
        app_data
    }
}
