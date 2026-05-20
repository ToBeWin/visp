use crate::error::{Result, VispError};
use std::path::PathBuf;
use std::process::Command;
use tauri::Emitter;

pub struct ModelDownloader {
    models_dir: PathBuf,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DownloadProgress {
    pub model_name: String,
    pub progress: f32,
    pub status: String,
}

impl ModelDownloader {
    pub fn new(models_dir: PathBuf) -> Self {
        Self { models_dir }
    }
    
    pub async fn download_all_models<R: tauri::Runtime>(
        &self,
        app: tauri::AppHandle<R>,
    ) -> Result<()> {
        // 确保模型目录存在
        std::fs::create_dir_all(&self.models_dir)?;
        
        tracing::info!("开始下载模型到: {:?}", self.models_dir);
        
        // 发送开始事件
        app.emit("download-start", ())
            .map_err(|e| VispError::ConfigError(format!("Failed to emit event: {}", e)))?;
        
        // 下载 Whisper 模型
        self.download_whisper_model(&app).await?;
        
        // 下载 Qwen 模型
        self.download_qwen_model(&app).await?;
        
        // 发送完成事件
        app.emit("download-complete", ())
            .map_err(|e| VispError::ConfigError(format!("Failed to emit event: {}", e)))?;
        
        tracing::info!("所有模型下载完成");
        Ok(())
    }
    
    async fn download_whisper_model<R: tauri::Runtime>(
        &self,
        app: &tauri::AppHandle<R>,
    ) -> Result<()> {
        tracing::info!("下载 Whisper 模型...");
        
        // 发送进度事件
        app.emit("download-progress", DownloadProgress {
            model_name: "Whisper".to_string(),
            progress: 0.0,
            status: "开始下载...".to_string(),
        }).ok();
        
        let whisper_path = self.models_dir.join("whisper-large-v3-turbo-q4_0.bin");
        
        // 如果文件已存在，跳过
        if whisper_path.exists() {
            tracing::info!("Whisper 模型已存在，跳过下载");
            app.emit("download-progress", DownloadProgress {
                model_name: "Whisper".to_string(),
                progress: 100.0,
                status: "已存在".to_string(),
            }).ok();
            return Ok(());
        }
        
        // 使用 huggingface-cli 下载
        let output = Command::new("huggingface-cli")
            .args(&[
                "download",
                "ggerganov/whisper.cpp",
                "ggml-large-v3-turbo-q4_0.bin",
                "--local-dir",
                self.models_dir.to_str().unwrap(),
                "--local-dir-use-symlinks",
                "False",
            ])
            .output();
        
        match output {
            Ok(output) => {
                if output.status.success() {
                    // 重命名文件
                    let downloaded_file = self.models_dir.join("ggml-large-v3-turbo-q4_0.bin");
                    if downloaded_file.exists() {
                        std::fs::rename(&downloaded_file, &whisper_path)?;
                    }
                    
                    tracing::info!("Whisper 模型下载完成");
                    app.emit("download-progress", DownloadProgress {
                        model_name: "Whisper".to_string(),
                        progress: 100.0,
                        status: "下载完成".to_string(),
                    }).ok();
                    Ok(())
                } else {
                    let error_msg = String::from_utf8_lossy(&output.stderr);
                    Err(VispError::ModelLoadError(format!("下载失败: {}", error_msg)))
                }
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    Err(VispError::ModelLoadError(
                        "未找到 huggingface-cli，请先安装: pip3 install -U \"huggingface_hub[cli]\"".to_string()
                    ))
                } else {
                    Err(VispError::from(e))
                }
            }
        }
    }
    
    async fn download_qwen_model<R: tauri::Runtime>(
        &self,
        app: &tauri::AppHandle<R>,
    ) -> Result<()> {
        tracing::info!("下载 Qwen 模型...");
        
        app.emit("download-progress", DownloadProgress {
            model_name: "Qwen".to_string(),
            progress: 0.0,
            status: "开始下载...".to_string(),
        }).ok();
        
        let qwen_path = self.models_dir.join("qwen3.5-0.8b-instruct-q4_k_m.gguf");
        
        if qwen_path.exists() {
            tracing::info!("Qwen 模型已存在，跳过下载");
            app.emit("download-progress", DownloadProgress {
                model_name: "Qwen".to_string(),
                progress: 100.0,
                status: "已存在".to_string(),
            }).ok();
            return Ok(());
        }
        
        let output = Command::new("huggingface-cli")
            .args(&[
                "download",
                "Qwen/Qwen2.5-0.5B-Instruct-GGUF",
                "qwen2.5-0.5b-instruct-q4_k_m.gguf",
                "--local-dir",
                self.models_dir.to_str().unwrap(),
                "--local-dir-use-symlinks",
                "False",
            ])
            .output();
        
        match output {
            Ok(output) => {
                if output.status.success() {
                    let downloaded_file = self.models_dir.join("qwen2.5-0.5b-instruct-q4_k_m.gguf");
                    if downloaded_file.exists() {
                        std::fs::rename(&downloaded_file, &qwen_path)?;
                    }
                    
                    tracing::info!("Qwen 模型下载完成");
                    app.emit("download-progress", DownloadProgress {
                        model_name: "Qwen".to_string(),
                        progress: 100.0,
                        status: "下载完成".to_string(),
                    }).ok();
                    Ok(())
                } else {
                    let error_msg = String::from_utf8_lossy(&output.stderr);
                    Err(VispError::ModelLoadError(format!("下载失败: {}", error_msg)))
                }
            }
            Err(e) => Err(VispError::from(e))
        }
    }
}
