use crate::error::{Result, VispError};
use std::path::PathBuf;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// 日志系统管理器
pub struct LoggingManager {
    log_dir: PathBuf,
}

impl LoggingManager {
    /// 创建新的日志管理器
    pub fn new() -> Result<Self> {
        let log_dir = Self::get_log_directory()?;
        
        // 确保日志目录存在
        std::fs::create_dir_all(&log_dir)
            .map_err(|e| VispError::ConfigError(format!("无法创建日志目录: {}", e)))?;
        
        Ok(Self { log_dir })
    }
    
    /// 获取日志目录路径
    fn get_log_directory() -> Result<PathBuf> {
        let log_dir = if cfg!(target_os = "windows") {
            dirs::data_local_dir()
                .ok_or_else(|| VispError::ConfigError("无法获取 Windows 数据目录".to_string()))?
                .join("Visp")
                .join("logs")
        } else if cfg!(target_os = "macos") {
            dirs::home_dir()
                .ok_or_else(|| VispError::ConfigError("无法获取 macOS 主目录".to_string()))?
                .join("Library")
                .join("Logs")
                .join("Visp")
        } else {
            // Linux
            dirs::data_local_dir()
                .ok_or_else(|| VispError::ConfigError("无法获取 Linux 数据目录".to_string()))?
                .join("visp")
                .join("logs")
        };
        
        Ok(log_dir)
    }
    
    /// 初始化日志系统
    pub fn init(&self, level: &str) -> Result<()> {
        // 创建文件 appender (每天轮转)
        let file_appender = RollingFileAppender::new(
            Rotation::DAILY,
            &self.log_dir,
            "visp.log",
        );
        
        // 解析日志级别
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new(level));
        
        // 配置日志订阅器
        tracing_subscriber::registry()
            .with(env_filter)
            .with(
                fmt::layer()
                    .with_writer(file_appender)
                    .with_ansi(false)
                    .with_target(true)
                    .with_thread_ids(true)
                    .with_line_number(true)
            )
            .with(
                fmt::layer()
                    .with_writer(std::io::stdout)
                    .with_ansi(true)
            )
            .init();
        
        tracing::info!("日志系统初始化完成，日志目录: {:?}", self.log_dir);
        Ok(())
    }
    
    /// 清理旧日志文件
    pub fn cleanup_old_logs(&self, days_to_keep: u64) -> Result<()> {
        tracing::info!("清理 {} 天前的日志文件", days_to_keep);
        
        let cutoff_time = std::time::SystemTime::now()
            - std::time::Duration::from_secs(days_to_keep * 24 * 60 * 60);
        
        let entries = std::fs::read_dir(&self.log_dir)
            .map_err(|e| VispError::ConfigError(format!("无法读取日志目录: {}", e)))?;
        
        let mut deleted_count = 0;
        
        for entry in entries {
            let entry = entry.map_err(|e| {
                VispError::ConfigError(format!("无法读取目录项: {}", e))
            })?;
            
            let metadata = entry.metadata().map_err(|e| {
                VispError::ConfigError(format!("无法读取文件元数据: {}", e))
            })?;
            
            if metadata.is_file() {
                if let Ok(modified) = metadata.modified() {
                    if modified < cutoff_time {
                        if let Err(e) = std::fs::remove_file(entry.path()) {
                            tracing::warn!("无法删除日志文件 {:?}: {}", entry.path(), e);
                        } else {
                            deleted_count += 1;
                            tracing::debug!("已删除旧日志文件: {:?}", entry.path());
                        }
                    }
                }
            }
        }
        
        tracing::info!("清理完成，删除了 {} 个旧日志文件", deleted_count);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_log_directory() {
        let result = LoggingManager::get_log_directory();
        assert!(result.is_ok());
        
        let log_dir = result.unwrap();
        assert!(log_dir.to_string_lossy().contains("Visp") || 
                log_dir.to_string_lossy().contains("visp"));
    }
    
    #[test]
    fn test_logging_manager_creation() {
        let manager = LoggingManager::new();
        assert!(manager.is_ok());
    }
}
