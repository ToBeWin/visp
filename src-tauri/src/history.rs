use rusqlite::Connection;
use serde::Serialize;
use std::path::Path;

use crate::error::{Result, VispError};

#[derive(Debug, Clone, Serialize)]
pub struct TranslationRecord {
    pub id: String,
    pub timestamp: String,
    pub source_text: String,
    pub translated_text: String,
    pub source_lang: String,
    pub target_lang: String,
    pub mode: String,
}

pub struct HistoryStore {
    conn: Connection,
}

impl HistoryStore {
    pub fn new(db_path: &Path) -> Result<Self> {
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| VispError::HistoryError(format!("无法创建历史数据库目录: {}", e)))?;
        }

        let conn = Connection::open(db_path)
            .map_err(|e| VispError::HistoryError(format!("无法打开历史数据库: {}", e)))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS translations (
                id TEXT PRIMARY KEY,
                timestamp TEXT NOT NULL,
                source_text TEXT NOT NULL,
                translated_text TEXT NOT NULL,
                source_lang TEXT NOT NULL,
                target_lang TEXT NOT NULL,
                mode TEXT NOT NULL
            )",
            [],
        ).map_err(|e| VispError::HistoryError(format!("无法创建历史表: {}", e)))?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_timestamp ON translations (timestamp)",
            [],
        ).ok();

        tracing::info!("翻译历史记录初始化完成: {:?}", db_path);
        Ok(Self { conn })
    }

    pub fn record(
        &self,
        source: &str,
        translated: &str,
        source_lang: &str,
        target_lang: &str,
        mode: &str,
    ) -> Result<String> {
        if source.trim().is_empty() {
            return Ok(String::new());
        }

        let id = format!("hist_{}", chrono::Utc::now().timestamp_millis());
        let timestamp = chrono::Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT INTO translations (id, timestamp, source_text, translated_text, source_lang, target_lang, mode)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            [&id, &timestamp, source, translated, source_lang, target_lang, mode],
        ).map_err(|e| VispError::HistoryError(format!("无法记录翻译: {}", e)))?;

        Ok(id)
    }

    pub fn list(&self, limit: usize, offset: usize) -> Result<Vec<TranslationRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, timestamp, source_text, translated_text, source_lang, target_lang, mode
             FROM translations
             ORDER BY timestamp DESC
             LIMIT ?1 OFFSET ?2"
        ).map_err(|e| VispError::HistoryError(format!("无法查询历史: {}", e)))?;

        let rows = stmt.query_map(&[&limit, &offset], |row| {
            Ok(TranslationRecord {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                source_text: row.get(2)?,
                translated_text: row.get(3)?,
                source_lang: row.get(4)?,
                target_lang: row.get(5)?,
                mode: row.get(6)?,
            })
        }).map_err(|e| VispError::HistoryError(format!("无法查询历史: {}", e)))?;

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| VispError::HistoryError(format!("无法读取历史结果: {}", e)))
    }

    #[allow(dead_code)]
    pub fn count(&self) -> Result<usize> {
        let count: i64 = self.conn
            .query_row("SELECT COUNT(*) FROM translations", [], |row| row.get(0))
            .map_err(|e| VispError::HistoryError(format!("无法统计记录数: {}", e)))?;
        Ok(count as usize)
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        self.conn.execute(
            "DELETE FROM translations WHERE id = ?1",
            [id],
        ).map_err(|e| VispError::HistoryError(format!("无法删除记录: {}", e)))?;
        Ok(())
    }

    pub fn clear(&self) -> Result<()> {
        self.conn.execute("DELETE FROM translations", [])
            .map_err(|e| VispError::HistoryError(format!("无法清空历史: {}", e)))?;
        Ok(())
    }

    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<TranslationRecord>> {
        let pattern = format!("%{}%", query);
        let mut stmt = self.conn.prepare(
            "SELECT id, timestamp, source_text, translated_text, source_lang, target_lang, mode
             FROM translations
             WHERE source_text LIKE ?1 OR translated_text LIKE ?1
             ORDER BY timestamp DESC
             LIMIT ?2"
        ).map_err(|e| VispError::HistoryError(format!("无法搜索历史: {}", e)))?;

        let rows = stmt.query_map((&pattern, &(limit as i64)), |row| {
            Ok(TranslationRecord {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                source_text: row.get(2)?,
                translated_text: row.get(3)?,
                source_lang: row.get(4)?,
                target_lang: row.get(5)?,
                mode: row.get(6)?,
            })
        }).map_err(|e| VispError::HistoryError(format!("无法搜索历史: {}", e)))?;

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| VispError::HistoryError(format!("无法读取搜索结果: {}", e)))
    }
}
