use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc::{Receiver, Sender};
use directories::ProjectDirs;

use crate::config::AppConfig;
use crate::error::{Result, VispError};
use crate::history::HistoryStore;
use crate::asr::AsrEngine;
use crate::translation::TranslationEngine;

pub struct AppState {
    pub config: Arc<RwLock<AppConfig>>,
    pub current_mode: Arc<Mutex<Option<Mode>>>,
    pub voice_session: Arc<Mutex<Option<VoiceSession>>>,
    pub history_store: Arc<Mutex<HistoryStore>>,
    pub engine_cache: Arc<Mutex<EngineCache>>,
}

pub struct EngineCache {
    pub asr: Option<Arc<Mutex<AsrEngine>>>,
    pub translator: Option<Arc<Mutex<TranslationEngine>>>,
}

pub enum Mode {
    Voice,
    Text,
}

pub struct VoiceSession {
    pub stop_tx: Sender<()>,
    pub result_rx: Receiver<std::result::Result<(Vec<f32>, u32), String>>,
    pub waveform: Arc<Mutex<Vec<f32>>>,
}

impl AppState {
    pub fn new() -> Result<Self> {
        let config = AppConfig::load()?;

        let proj_dirs = ProjectDirs::from("live", "visp", "translator")
            .ok_or_else(|| VispError::ConfigError("无法确定数据目录".to_string()))?;
        let db_path = proj_dirs.data_dir().join("history.db");
        let history_store = HistoryStore::new(&db_path)?;

        // Preload engines in background thread for faster first invocation
        let engine_cache = Arc::new(Mutex::new(EngineCache {
            asr: None,
            translator: None,
        }));

        let cache_for_thread = Arc::clone(&engine_cache);
        std::thread::spawn(move || {
            tracing::info!("后台预加载翻译引擎...");
            if let Ok(engine) = TranslationEngine::new() {
                if let Ok(mut cache) = cache_for_thread.lock() {
                    cache.translator = Some(Arc::new(Mutex::new(engine)));
                    tracing::info!("翻译引擎预加载完成");
                }
            } else {
                tracing::warn!("翻译引擎预加载失败，将在使用时初始化");
            }
        });

        let cache_for_asr = Arc::clone(&engine_cache);
        std::thread::spawn(move || {
            tracing::info!("后台预加载 ASR 引擎...");
            if let Ok(engine) = AsrEngine::new() {
                if let Ok(mut cache) = cache_for_asr.lock() {
                    cache.asr = Some(Arc::new(Mutex::new(engine)));
                    tracing::info!("ASR 引擎预加载完成");
                }
            } else {
                tracing::warn!("ASR 引擎预加载失败，将在使用时初始化");
            }
        });

        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            current_mode: Arc::new(Mutex::new(None)),
            voice_session: Arc::new(Mutex::new(None)),
            history_store: Arc::new(Mutex::new(history_store)),
            engine_cache: Arc::new(Mutex::new(EngineCache {
                asr: None,
                translator: None,
            })),
        })
    }

    /// Get or initialize ASR engine through the cache. If the background preload already
    /// loaded one, the cached Arc is returned. Otherwise, a new engine is created.
    pub fn get_or_init_asr(&self) -> std::result::Result<Arc<Mutex<AsrEngine>>, String> {
        let mut cache = self.engine_cache.lock()
            .map_err(|e| format!("Failed to lock engine cache: {}", e))?;
        if cache.asr.is_none() {
            let engine = AsrEngine::new()
                .map_err(|e| format!("ASR engine init failed: {}", e))?;
            cache.asr = Some(Arc::new(Mutex::new(engine)));
        }
        Ok(cache.asr.clone().unwrap())
    }

    /// Get or initialize translation engine through the cache.
    pub fn get_or_init_translator(&self) -> std::result::Result<Arc<Mutex<TranslationEngine>>, String> {
        let mut cache = self.engine_cache.lock()
            .map_err(|e| format!("Failed to lock engine cache: {}", e))?;
        if cache.translator.is_none() {
            let engine = TranslationEngine::new()
                .map_err(|e| format!("Translation engine init failed: {}", e))?;
            cache.translator = Some(Arc::new(Mutex::new(engine)));
        }
        Ok(cache.translator.clone().unwrap())
    }
}
