use crate::error::{Result, VispError};
use directories::ProjectDirs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct ModelLoader {
    models_dir: PathBuf,
}

#[derive(Debug)]
pub struct ModelStatus {
    pub asr_available: bool,
    pub translation_available: bool,
    pub missing_files: Vec<String>,
    pub asr_summary: String,
    pub asr_detail: String,
    pub translation_summary: String,
    pub translation_detail: String,
    pub download_supported: bool,
}

impl ModelLoader {
    pub fn new() -> Result<Self> {
        let proj_dirs = ProjectDirs::from("live", "visp", "translator")
            .ok_or_else(|| VispError::ConfigError("无法确定模型目录".to_string()))?;
        
        let models_dir = proj_dirs.data_dir().join("models");
        std::fs::create_dir_all(&models_dir)?;
        
        Ok(Self { models_dir })
    }
    
    pub fn check_models(&self) -> ModelStatus {
        let asr_path = self.get_asr_model_path();
        let translation_path = self.get_translation_model_path();
        let _asr_tokenizer_path = self.get_asr_tokenizer_path();
        let translation_tokenizer_path = self.get_translation_tokenizer_path();
        let ollama_model = Self::detect_ollama_translation_model();

        let mut missing_files = Vec::new();

        let (asr_available, asr_summary, asr_detail) = if Self::apple_speech_available() {
            (
                true,
                "已就绪".to_string(),
                "当前版本在 macOS 上使用系统语音识别，不需要下载 Whisper 模型。权限会在引导页单独处理。".to_string(),
            )
        } else if asr_path.exists() {
            (
                true,
                "已就绪".to_string(),
                format!("已检测到 Whisper 模型，将使用 whisper-rs (whisper.cpp) 进行本地语音识别。"),
            )
        } else {
            missing_files.push("whisper-large-v3-turbo".to_string());
            (
                false,
                "未就绪".to_string(),
                "非 macOS 平台需要 Whisper 模型文件才能进行语音识别。请放置 .bin 或 .gguf 格式的 Whisper 模型到模型目录。".to_string(),
            )
        };

        // Also check for Qwen2.5 variant (Candle's quantized_qwen2 supports Qwen2/Qwen2.5)
        let fallback_model_path = self.models_dir.join("qwen2.5-0.5b-instruct-q4_k_m.gguf");
        let fallback_tokenizer_path = self.models_dir.join("tokenizer.json");

        let (translation_available, translation_summary, translation_detail) =
            if translation_path.exists() && translation_tokenizer_path.exists() {
                (
                    true,
                    "已就绪".to_string(),
                    "已检测到本地 GGUF 模型和 tokenizer，将使用内置 Candle 推理引擎进行翻译。".to_string(),
                )
            } else if fallback_model_path.exists() && fallback_tokenizer_path.exists() {
                (
                    true,
                    "已就绪".to_string(),
                    "已检测到 Qwen2.5 GGUF 模型，将使用内置 Candle 推理引擎进行翻译。".to_string(),
                )
            } else if let Some(model) = &ollama_model {
                (
                    true,
                    "已就绪".to_string(),
                    format!("当前使用本机 Ollama 模型 {} 进行翻译。", model),
                )
            } else {
                if !translation_path.exists() && !fallback_model_path.exists() {
                    missing_files.push("qwen2.5-0.5b-instruct-q4_k_m.gguf".to_string());
                }
                if !translation_tokenizer_path.exists() && !fallback_tokenizer_path.exists() {
                    missing_files.push("qwen-tokenizer.json or tokenizer.json".to_string());
                }
                (
                    false,
                    "未就绪".to_string(),
                    "还没有可用的翻译后端。请先放置 GGUF 模型和 tokenizer.json 到模型目录，或安装并启动 Ollama 并拉取相应模型。".to_string(),
                )
            };

        let download_supported = {
            let has_ollama = ollama_model.is_some();
            // macOS gets ASR via system Speech, other platforms need Whisper model files
            #[cfg(target_os = "macos")]
            { has_ollama }
            #[cfg(not(target_os = "macos"))]
            { has_ollama }
        };

        ModelStatus {
            asr_available,
            translation_available,
            missing_files,
            asr_summary,
            asr_detail,
            translation_summary,
            translation_detail,
            download_supported,
        }
    }

    pub fn get_models_dir(&self) -> &Path {
        &self.models_dir
    }

    pub fn get_asr_model_path(&self) -> PathBuf {
        self.models_dir.join("whisper-large-v3-turbo-q4_0.bin")
    }

    pub fn get_asr_tokenizer_path(&self) -> PathBuf {
        self.models_dir.join("whisper-tokenizer.json")
    }

    pub fn get_translation_model_path(&self) -> PathBuf {
        self.models_dir.join("qwen3.5-0.8b-instruct-q4_k_m.gguf")
    }

    pub fn get_translation_tokenizer_path(&self) -> PathBuf {
        self.models_dir.join("qwen-tokenizer.json")
    }

    pub fn verify_model(&self, path: &Path) -> Result<()> {
        if !path.exists() {
            return Err(VispError::ModelLoadError(
                format!("模型文件不存在: {:?}", path)
            ));
        }
        
        // TODO: 实现更详细的完整性验证
        Ok(())
    }

    pub fn ollama_cli_available() -> bool {
        Command::new("ollama")
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    pub fn ollama_service_reachable() -> bool {
        Command::new("ollama")
            .arg("list")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    pub fn detect_ollama_translation_model() -> Option<String> {
        let output = Command::new("ollama").arg("list").output();
        let Ok(output) = output else {
            return None;
        };

        if !output.status.success() {
            return None;
        }

        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        for candidate in [
            "translategemma:latest",
            "qwen3.5:0.8b",
            "qwen3.5:2b",
            "qwen3.5:9b",
        ] {
            if stdout.contains(candidate) {
                return Some(candidate.to_string());
            }
        }

        None
    }

    fn apple_speech_available() -> bool {
        #[cfg(target_os = "macos")]
        {
            true
        }

        #[cfg(not(target_os = "macos"))]
        {
            false
        }
    }
}
