use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use directories::ProjectDirs;
use crate::error::{Result, VispError};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub hotkeys: HotkeyConfig,
    pub translation: TranslationConfig,
    pub ui: UiConfig,
    pub logging: LoggingConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HotkeyConfig {
    pub voice_mode: String,
    pub text_mode: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TranslationConfig {
    pub direction: TranslationDirection,
    pub preserve_formatting: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TranslationDirection {
    Auto,
    ChineseToEnglish,
    EnglishToChinese,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UiConfig {
    pub opacity: f32,
    pub animation_speed: f32,
    pub auto_dismiss_timeout: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoggingConfig {
    pub level: LogLevel,
    pub max_file_size: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            hotkeys: HotkeyConfig {
                voice_mode: default_voice_hotkey().to_string(),
                text_mode: default_text_hotkey().to_string(),
            },
            translation: TranslationConfig {
                direction: TranslationDirection::Auto,
                preserve_formatting: true,
            },
            ui: UiConfig {
                opacity: 0.95,
                animation_speed: 1.0,
                auto_dismiss_timeout: 10,
            },
            logging: LoggingConfig {
                level: LogLevel::Info,
                max_file_size: 50,
            },
        }
    }
}

impl AppConfig {
    /// 获取配置文件路径
    pub fn config_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("live", "visp", "translator")
            .ok_or_else(|| VispError::ConfigError("无法确定配置目录".to_string()))?;
        
        let config_dir = proj_dirs.config_dir();
        std::fs::create_dir_all(config_dir)?;
        
        Ok(config_dir.join("config.json"))
    }
    
    /// 加载配置
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        
        if !path.exists() {
            let config = Self::default();
            config.save()?;
            return Ok(config);
        }
        
        let content = std::fs::read_to_string(&path)?;
        let mut config: AppConfig = serde_json::from_str(&content)?;
        let mut migrated = false;

        if cfg!(target_os = "macos") {
            if config.hotkeys.voice_mode == "Alt+V" {
                config.hotkeys.voice_mode = default_voice_hotkey().to_string();
                migrated = true;
            }

            if config.hotkeys.text_mode == "Alt+C" {
                config.hotkeys.text_mode = default_text_hotkey().to_string();
                migrated = true;
            }
        }

        if migrated {
            config.save()?;
        }

        Ok(config)
    }
    
    /// 保存配置
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, content)?;
        Ok(())
    }
}

fn default_voice_hotkey() -> &'static str {
    if cfg!(target_os = "macos") {
        "CommandOrControl+Shift+V"
    } else {
        "Ctrl+Shift+V"
    }
}

fn default_text_hotkey() -> &'static str {
    if cfg!(target_os = "macos") {
        "CommandOrControl+Shift+C"
    } else {
        "Ctrl+Shift+C"
    }
}
