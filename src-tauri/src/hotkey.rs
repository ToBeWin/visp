use tauri::{AppHandle, Emitter};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};
use crate::error::{Result, VispError};
use std::str::FromStr;

/// 快捷键管理器 - 使用 Tauri global-shortcut 插件
pub struct HotkeyManager {
    app_handle: AppHandle,
}

impl HotkeyManager {
    /// 创建新的快捷键管理器实例
    pub fn new(app_handle: AppHandle) -> Self {
        tracing::info!("初始化快捷键管理器");
        
        Self { app_handle }
    }
    
    /// 注册语音模式快捷键
    pub fn register_voice_mode(&self, hotkey: &str) -> Result<()> {
        tracing::info!("注册语音模式快捷键: {}", hotkey);
        
        let shortcut = Shortcut::from_str(hotkey)
            .map_err(|e| VispError::HotkeyError(format!("无效的快捷键格式: {}", e)))?;
        
        self.app_handle
            .global_shortcut()
            .on_shortcut(shortcut, move |app, _shortcut, event| {
                if event.state() == ShortcutState::Pressed {
                    tracing::info!("语音模式快捷键切换");
                    let _ = app.emit("voice-mode-toggle", ());
                }
            })
            .map_err(|e| VispError::HotkeyError(format!("注册快捷键失败: {}", e)))?;
        
        tracing::info!("语音模式快捷键注册成功");
        Ok(())
    }
    
    /// 注册文本模式快捷键 (Alt+C)
    pub fn register_text_mode(&self, hotkey: &str) -> Result<()> {
        tracing::info!("注册文本模式快捷键: {}", hotkey);
        
        let shortcut = Shortcut::from_str(hotkey)
            .map_err(|e| VispError::HotkeyError(format!("无效的快捷键格式: {}", e)))?;
        
        self.app_handle
            .global_shortcut()
            .on_shortcut(shortcut, move |app, _shortcut, event| {
                if event.state() == ShortcutState::Pressed {
                    tracing::info!("文本模式快捷键触发");
                    // 发送事件到前端
                    let _ = app.emit("text-mode-trigger", ());
                }
            })
            .map_err(|e| VispError::HotkeyError(format!("注册快捷键失败: {}", e)))?;
        
        tracing::info!("文本模式快捷键注册成功");
        Ok(())
    }
    
    /// 注销所有快捷键
    pub fn unregister_all(&self) -> Result<()> {
        tracing::info!("注销所有快捷键");
        
        self.app_handle
            .global_shortcut()
            .unregister_all()
            .map_err(|e| VispError::HotkeyError(format!("注销所有快捷键失败: {}", e)))?;
        
        tracing::info!("所有快捷键已注销");
        Ok(())
    }
    
}

/// 初始化全局快捷键
pub fn init_global_shortcuts(app_handle: &AppHandle, voice_hotkey: &str, text_hotkey: &str) -> Result<()> {
    tracing::info!("初始化全局快捷键");

    let manager = HotkeyManager::new(app_handle.clone());

    manager.unregister_all().ok();
    manager.register_voice_mode(voice_hotkey)?;
    manager.register_text_mode(text_hotkey)?;

    tracing::info!("全局快捷键初始化完成");
    Ok(())
}

pub fn apply_hotkey_config(
    app_handle: &AppHandle,
    voice_hotkey: &str,
    text_hotkey: &str,
) -> Result<()> {
    init_global_shortcuts(app_handle, voice_hotkey, text_hotkey)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mac_friendly_hotkeys() {
        assert!(Shortcut::from_str("CommandOrControl+Shift+V").is_ok());
        assert!(Shortcut::from_str("CommandOrControl+Shift+C").is_ok());
    }

    #[test]
    fn test_parse_special_keys() {
        assert!(Shortcut::from_str("Space").is_ok());
        assert!(Shortcut::from_str("F6").is_ok());
        assert!(Shortcut::from_str("Fn").is_err());
    }
}
