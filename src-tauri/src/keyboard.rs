use enigo::{Enigo, Key, Keyboard, Settings};
use crate::error::{Result, VispError};

/// 键盘模拟器 - 使用 enigo 模拟键盘输入
pub struct KeyboardSimulator {
    enigo: Enigo,
}

impl KeyboardSimulator {
    /// 创建新的键盘模拟器实例
    pub fn new() -> Result<Self> {
        tracing::info!("初始化键盘模拟器");
        
        let enigo = Enigo::new(&Settings::default())
            .map_err(|e| VispError::KeyboardError(format!("无法创建 Enigo: {}", e)))?;
        
        Ok(Self { enigo })
    }
    
    /// 模拟 Ctrl+V 粘贴操作
    pub fn paste(&mut self) -> Result<()> {
        tracing::debug!("模拟 Ctrl+V 操作");
        
        // 按下 Ctrl (macOS 使用 Cmd)
        #[cfg(target_os = "macos")]
        {
            self.enigo.key(Key::Meta, enigo::Direction::Press)
                .map_err(|e| VispError::KeyboardError(format!("按键失败: {}", e)))?;
            self.enigo.key(Key::Unicode('v'), enigo::Direction::Click)
                .map_err(|e| VispError::KeyboardError(format!("按键失败: {}", e)))?;
            self.enigo.key(Key::Meta, enigo::Direction::Release)
                .map_err(|e| VispError::KeyboardError(format!("按键失败: {}", e)))?;
        }
        #[cfg(not(target_os = "macos"))]
        {
            self.enigo.key(Key::Control, enigo::Direction::Press)
                .map_err(|e| VispError::KeyboardError(format!("按键失败: {}", e)))?;
            self.enigo.key(Key::Unicode('v'), enigo::Direction::Click)
                .map_err(|e| VispError::KeyboardError(format!("按键失败: {}", e)))?;
            self.enigo.key(Key::Control, enigo::Direction::Release)
                .map_err(|e| VispError::KeyboardError(format!("按键失败: {}", e)))?;
        }
        
        tracing::debug!("Ctrl+V 模拟完成");
        Ok(())
    }
    
    /// 快速输入文本（通过剪贴板）
    pub fn type_text_fast(&mut self, text: &str) -> Result<()> {
        tracing::info!("快速输入文本（通过剪贴板），长度: {} 字符", text.len());
        
        use crate::clipboard::ClipboardManager;
        
        // 备份剪贴板
        let clipboard = ClipboardManager::new()?;
        clipboard.backup()?;
        
        // 写入文本到剪贴板
        clipboard.write_text(text)?;
        
        // 粘贴
        self.paste()?;
        
        // 恢复剪贴板
        clipboard.restore()?;
        
        tracing::info!("快速输入完成");
        Ok(())
    }
}

impl Default for KeyboardSimulator {
    fn default() -> Self {
        Self::new().expect("无法创建键盘模拟器")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_keyboard_simulator_creation() {
        let result = KeyboardSimulator::new();
        assert!(result.is_ok());
    }
}
