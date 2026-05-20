use arboard::Clipboard;
use crate::error::{Result, VispError};
use std::sync::Mutex;

/// 剪贴板管理器 - 使用 arboard 进行剪贴板操作
pub struct ClipboardManager {
    clipboard: Mutex<Clipboard>,
    backup: Mutex<Option<String>>,
}

impl ClipboardManager {
    /// 创建新的剪贴板管理器实例
    pub fn new() -> Result<Self> {
        tracing::info!("初始化剪贴板管理器");
        
        let clipboard = Clipboard::new()
            .map_err(|e| VispError::ClipboardError(format!("无法初始化剪贴板: {}", e)))?;
        
        Ok(Self {
            clipboard: Mutex::new(clipboard),
            backup: Mutex::new(None),
        })
    }
    
    /// 读取剪贴板文本
    pub fn read_text(&self) -> Result<String> {
        tracing::debug!("读取剪贴板内容");
        
        let mut clipboard = self.clipboard.lock()
            .map_err(|e| VispError::ClipboardError(format!("锁定剪贴板失败: {}", e)))?;
        
        let text = clipboard.get_text()
            .map_err(|e| VispError::ClipboardError(format!("读取剪贴板失败: {}", e)))?;
        
        tracing::debug!("剪贴板内容长度: {} 字符", text.len());
        Ok(text)
    }
    
    /// 写入文本到剪贴板
    pub fn write_text(&self, text: &str) -> Result<()> {
        tracing::debug!("写入剪贴板，长度: {} 字符", text.len());
        
        let mut clipboard = self.clipboard.lock()
            .map_err(|e| VispError::ClipboardError(format!("锁定剪贴板失败: {}", e)))?;
        
        clipboard.set_text(text)
            .map_err(|e| VispError::ClipboardError(format!("写入剪贴板失败: {}", e)))?;
        
        tracing::debug!("剪贴板写入成功");
        Ok(())
    }
    
    /// 备份当前剪贴板内容
    pub fn backup(&self) -> Result<()> {
        tracing::debug!("备份剪贴板内容");
        
        let text = self.read_text().unwrap_or_default();
        
        let mut backup = self.backup.lock()
            .map_err(|e| VispError::ClipboardError(format!("锁定备份失败: {}", e)))?;
        
        *backup = Some(text);
        tracing::debug!("剪贴板备份完成");
        Ok(())
    }
    
    /// 恢复备份的剪贴板内容
    pub fn restore(&self) -> Result<()> {
        tracing::debug!("恢复剪贴板内容");
        
        let backup = self.backup.lock()
            .map_err(|e| VispError::ClipboardError(format!("锁定备份失败: {}", e)))?;
        
        if let Some(text) = backup.as_ref() {
            self.write_text(text)?;
            tracing::debug!("剪贴板恢复完成");
        } else {
            tracing::warn!("没有备份可恢复");
        }
        
        Ok(())
    }
    
    /// 模拟 Ctrl+C 操作（通过键盘模拟）
    pub fn simulate_copy() -> Result<()> {
        tracing::debug!("模拟 Ctrl+C 操作");
        
        use enigo::{Enigo, Key, Keyboard, Settings};
        let mut enigo = Enigo::new(&Settings::default())
            .map_err(|e| VispError::ClipboardError(format!("无法创建 Enigo: {}", e)))?;
        
        // 按下 Ctrl+C
        #[cfg(target_os = "macos")]
        {
            enigo.key(Key::Meta, enigo::Direction::Press)
                .map_err(|e| VispError::ClipboardError(format!("按键失败: {}", e)))?;
            enigo.key(Key::Unicode('c'), enigo::Direction::Click)
                .map_err(|e| VispError::ClipboardError(format!("按键失败: {}", e)))?;
            enigo.key(Key::Meta, enigo::Direction::Release)
                .map_err(|e| VispError::ClipboardError(format!("按键失败: {}", e)))?;
        }
        #[cfg(not(target_os = "macos"))]
        {
            enigo.key(Key::Control, enigo::Direction::Press)
                .map_err(|e| VispError::ClipboardError(format!("按键失败: {}", e)))?;
            enigo.key(Key::Unicode('c'), enigo::Direction::Click)
                .map_err(|e| VispError::ClipboardError(format!("按键失败: {}", e)))?;
            enigo.key(Key::Control, enigo::Direction::Release)
                .map_err(|e| VispError::ClipboardError(format!("按键失败: {}", e)))?;
        }
        
        // 等待剪贴板更新
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        tracing::debug!("Ctrl+C 模拟完成");
        Ok(())
    }
}

impl Default for ClipboardManager {
    fn default() -> Self {
        Self::new().expect("无法创建剪贴板管理器")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[serial(clipboard)]
    #[test]
    fn test_clipboard_creation() {
        let result = ClipboardManager::new();
        assert!(result.is_ok());
    }

    #[serial(clipboard)]
    #[test]
    fn test_clipboard_read_write() {
        let manager = ClipboardManager::new().unwrap();

        let test_text = "Hello, Visp!";
        manager.write_text(test_text).unwrap();

        let read_text = manager.read_text().unwrap();
        assert_eq!(read_text, test_text);
    }

    #[serial(clipboard)]
    #[test]
    fn test_clipboard_backup_restore() {
        let manager = ClipboardManager::new().unwrap();

        // 写入原始内容
        let original = "Original content";
        manager.write_text(original).unwrap();

        // 备份
        manager.backup().unwrap();

        // 修改内容
        manager.write_text("New content").unwrap();

        // 恢复
        manager.restore().unwrap();

        // 验证
        let restored = manager.read_text().unwrap();
        assert_eq!(restored, original);
    }
}
