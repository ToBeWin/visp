use thiserror::Error;

#[derive(Error, Debug)]
pub enum VispError {
    #[error("模型加载失败: {0}")]
    ModelLoadError(String),
    
    #[error("音频捕获失败: {0}")]
    AudioCaptureError(String),
    
    #[error("语音识别失败: {0}")]
    AsrError(String),
    
    #[error("翻译失败: {0}")]
    TranslationError(String),
    
    #[error("剪贴板操作失败: {0}")]
    ClipboardError(String),
    
    #[error("键盘模拟失败: {0}")]
    KeyboardError(String),
    
    #[error("快捷键注册失败: {0}")]
    HotkeyError(String),
    
    #[error("配置错误: {0}")]
    ConfigError(String),
    
    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("序列化错误: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("历史记录错误: {0}")]
    HistoryError(String),

    #[error("Candle 推理错误: {0}")]
    CandleError(String),
}

pub type Result<T> = std::result::Result<T, VispError>;
