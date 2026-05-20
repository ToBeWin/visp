use crate::error::{Result, VispError};
use crate::inference::candle_backend::CandleInference;
use crate::models::ModelLoader;
use std::collections::HashSet;
use std::process::Command;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::SystemTime;

/// 翻译引擎 - 优先使用本地 Candle (GGUF) 模型，回退到 Ollama
pub struct TranslationEngine {
    backend: TranslationBackend,
    context_memory: Arc<Mutex<TranslationContextMemory>>,
}

enum TranslationBackend {
    Candle { inference: Arc<CandleInference> },
    Ollama { model: String },
    Unavailable { reason: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Chinese,
    English,
    Auto,
}

/// A single recent translation record kept in memory.
#[derive(Clone)]
pub struct RecentTranslationRecord {
    source: String,
    target: String,
    #[allow(dead_code)]
    timestamp: SystemTime,
}

/// LRU-style context memory for recent translations.
/// Keeps up to `max_entries` records, trimming the oldest when full.
pub struct TranslationContextMemory {
    recent: Vec<RecentTranslationRecord>,
    max_entries: usize,
}

impl TranslationContextMemory {
    pub fn new(max_entries: usize) -> Self {
        Self {
            recent: Vec::with_capacity(max_entries),
            max_entries,
        }
    }

    /// Add a translation record. If the buffer is full, the oldest entry is dropped.
    pub fn add(&mut self, source: &str, target: &str) {
        self.recent.push(RecentTranslationRecord {
            source: source.to_string(),
            target: target.to_string(),
            timestamp: SystemTime::now(),
        });
        // Trim to max_entries (keep the newest entries)
        if self.recent.len() > self.max_entries {
            let excess = self.recent.len() - self.max_entries;
            self.recent.drain(0..excess);
        }
    }

    /// Tokenize text into a set of lowercase word tokens for similarity comparison.
    fn tokenize(text: &str) -> HashSet<String> {
        text.split_whitespace()
            .map(|w| w.to_lowercase())
            .collect()
    }

    /// Compute Jaccard similarity between two token sets.
    fn jaccard(a: &HashSet<String>, b: &HashSet<String>) -> f32 {
        if a.is_empty() && b.is_empty() {
            return 0.0;
        }
        let intersection = a.intersection(b).count() as f32;
        let union = a.union(b).count() as f32;
        if union == 0.0 {
            0.0
        } else {
            intersection / union
        }
    }

    /// Find recent translations that share significant token overlap with the given text.
    /// Returns up to `limit` records sorted by similarity (highest first).
    fn find_similar(&self, text: &str, threshold: f32, limit: usize) -> Vec<&RecentTranslationRecord> {
        let query_tokens = Self::tokenize(text);
        if query_tokens.is_empty() {
            return Vec::new();
        }

        let mut scored: Vec<(&RecentTranslationRecord, f32)> = self
            .recent
            .iter()
            .map(|r| {
                let source_tokens = Self::tokenize(&r.source);
                let sim = Self::jaccard(&query_tokens, &source_tokens);
                (r, sim)
            })
            .filter(|(_, sim)| *sim >= threshold)
            .collect();

        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scored.into_iter().take(limit).map(|(r, _)| r).collect()
    }

    /// Build a formatted prompt section with similar past translations.
    /// Returns an empty string if no similar translations are found.
    pub fn get_prompt_context(&self, text: &str) -> String {
        let similar = self.find_similar(text, 0.3, 3);
        if similar.is_empty() {
            return String::new();
        }

        let mut section = String::from("参考翻译 (Reference translations):\n");
        for (i, record) in similar.iter().enumerate() {
            section.push_str(&format!(
                "{}. {} -> {}\n",
                i + 1,
                record.source,
                record.target
            ));
        }
        section.push_str("Use these references for terminology consistency.\n\n");
        section
    }
}

impl Default for TranslationContextMemory {
    fn default() -> Self {
        Self::new(50)
    }
}

impl TranslationEngine {
    /// 创建新的翻译引擎实例。优先本地 Candle GGUF，其次 Ollama
    pub fn new() -> Result<Self> {
        Self::with_context_memory(TranslationContextMemory::default())
    }

    /// 创建带指定上下文记忆的翻译引擎实例。
    pub fn with_context_memory(context_memory: TranslationContextMemory) -> Result<Self> {
        tracing::info!("初始化翻译引擎...");

        let model_loader = ModelLoader::new()?;
        let backend = Self::resolve_backend(&model_loader);
        tracing::info!("翻译引擎初始化完成");

        Ok(Self {
            backend,
            context_memory: Arc::new(Mutex::new(context_memory)),
        })
    }

    fn resolve_backend(model_loader: &ModelLoader) -> TranslationBackend {
        let model_path = model_loader.get_translation_model_path();
        let tokenizer_path = model_loader.get_translation_tokenizer_path();

        // 1. 本地 Candle GGUF (Qwen3.5)
        if model_path.exists() && tokenizer_path.exists() {
            if let Ok(()) = model_loader.verify_model(&model_path) {
                match CandleInference::new(&model_path, &tokenizer_path) {
                    Ok(inference) => {
                        tracing::info!("使用本地 Candle GGUF 翻译后端: {:?}", model_path);
                        return TranslationBackend::Candle { inference: Arc::new(inference) };
                    }
                    Err(e) => {
                        tracing::warn!("Candle 模型加载失败: {}", e);
                    }
                }
            }
        }

        // 1b. 本地 Candle GGUF (Qwen2.5 变体)
        let models_dir = model_loader.get_models_dir();
        let fallback_model = models_dir.join("qwen2.5-0.5b-instruct-q4_k_m.gguf");
        let fallback_tokenizer = models_dir.join("tokenizer.json");
        if fallback_model.exists() && fallback_tokenizer.exists() {
            match CandleInference::new(&fallback_model, &fallback_tokenizer) {
                Ok(inference) => {
                    tracing::info!("使用 Qwen2.5 Candle GGUF 翻译后端: {:?}", fallback_model);
                    return TranslationBackend::Candle { inference: Arc::new(inference) };
                }
                Err(e) => {
                    tracing::warn!("Qwen2.5 Candle 模型加载失败: {}", e);
                }
            }
        }

        // 2. Ollama
        if let Some(model) = ModelLoader::detect_ollama_translation_model() {
            tracing::info!("回退到 Ollama 翻译后端: {}", model);
            return TranslationBackend::Ollama { model };
        }

        // 3. 不可用
        let reason = "未检测到可用翻译后端。请先安装并启动 Ollama，或将 GGUF 模型与 tokenizer.json 放置于模型目录以启用本地推理。".to_string();
        TranslationBackend::Unavailable { reason }
    }

    /// 翻译文本
    pub fn translate(&self, text: &str, source_lang: Language, target_lang: Language) -> Result<String> {
        tracing::info!("开始翻译: {:?} -> {:?}", source_lang, target_lang);
        tracing::debug!("输入文本: {}", text);

        let detected_source = if source_lang == Language::Auto {
            Self::detect_language(text)
        } else {
            source_lang
        };

        let final_target = if target_lang == Language::Auto {
            match detected_source {
                Language::Chinese => Language::English,
                Language::English => Language::Chinese,
                Language::Auto => Language::English,
            }
        } else {
            target_lang
        };

        tracing::info!("翻译方向: {:?} -> {:?}", detected_source, final_target);

        let context_section = self.context_memory.lock()
            .map(|cm| cm.get_prompt_context(text))
            .unwrap_or_default();

        let prompt = Self::build_prompt(text, detected_source, final_target, &context_section);
        let translation = self.generate(&prompt)?;
        let processed = Self::post_process(&translation, text);

        if let Ok(mut cm) = self.context_memory.lock() {
            cm.add(text, &processed);
        }

        tracing::info!("翻译完成");
        tracing::debug!("输出文本: {}", processed);

        Ok(processed)
    }

    /// 检测文本语言
    pub fn detect_language(text: &str) -> Language {
        let chinese_chars = text.chars()
            .filter(|c| {
                let code = *c as u32;
                (0x4E00..=0x9FFF).contains(&code)
                    || (0x3400..=0x4DBF).contains(&code)
                    || (0x20000..=0x2A6DF).contains(&code)
            })
            .count();

        let total_chars = text.chars().filter(|c| !c.is_whitespace()).count();

        if total_chars == 0 {
            return Language::Auto;
        }

        if chinese_chars as f32 / total_chars as f32 >= 0.25 {
            Language::Chinese
        } else {
            Language::English
        }
    }

    /// 构建翻译 prompt
    fn build_prompt(text: &str, source: Language, target: Language, context: &str) -> String {
        let base = match (source, target) {
            (Language::Chinese, Language::English) => format!(
                "Translate the following Chinese text into natural English. Output translation only, keep formatting and line breaks, keep code and proper nouns unchanged when appropriate.\n\n{}",
                text
            ),
            (Language::English, Language::Chinese) => format!(
                "Translate the following English text into natural Simplified Chinese. Output translation only, keep formatting and line breaks, keep code and proper nouns unchanged when appropriate.\n\n{}",
                text
            ),
            _ => format!(
                "Translate the following text into the most appropriate counterpart language between Simplified Chinese and English. Output translation only and preserve formatting.\n\n{}",
                text
            ),
        };

        if context.is_empty() {
            base
        } else {
            // Prepend context examples before the translation instruction
            format!("{}\n{}", context, base)
        }
    }

    fn generate(&self, prompt: &str) -> Result<String> {
        tracing::debug!("运行推理");

        match &self.backend {
            TranslationBackend::Candle { inference } => inference.generate(prompt, 256),
            TranslationBackend::Ollama { model } => self.generate_with_ollama(model, prompt),
            TranslationBackend::Unavailable { reason } => {
                Err(VispError::TranslationError(reason.clone()))
            }
        }
    }

    fn post_process(translation: &str, _original: &str) -> String {
        let mut result = translation.trim().to_string();

        // 移除可能的 markdown 代码块标记
        if result.starts_with("```") {
            result = result.trim_start_matches("```").trim().to_string();
        }
        if result.ends_with("```") {
            result = result.trim_end_matches("```").trim().to_string();
        }

        result
    }

    fn generate_with_ollama(&self, model: &str, prompt: &str) -> Result<String> {
        let output = Command::new("ollama")
            .args(["run", model, prompt])
            .output()
            .map_err(|e| {
                VispError::TranslationError(format!("无法调用 Ollama: {}", e))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            return Err(VispError::TranslationError(format!(
                "Ollama 执行失败: {}",
                if stderr.is_empty() { "未知错误".to_string() } else { stderr }
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let cleaned = Self::extract_final_response(&stdout);
        if cleaned.is_empty() {
            return Err(VispError::TranslationError(
                "Ollama 返回了空结果，请确认本地模型可正常工作。".to_string(),
            ));
        }

        Ok(cleaned)
    }

    fn extract_final_response(raw: &str) -> String {
        let stripped = Self::strip_ansi(raw);
        stripped
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .filter(|line| !line.eq_ignore_ascii_case("thinking..."))
            .filter(|line| !line.eq_ignore_ascii_case("thinking process"))
            .last()
            .unwrap_or_default()
            .to_string()
    }

    fn strip_ansi(raw: &str) -> String {
        let mut output = String::with_capacity(raw.len());
        let mut chars = raw.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\u{1b}' {
                match chars.peek().copied() {
                    Some('[') => {
                        chars.next();
                        while let Some(next) = chars.next() {
                            if ('@'..='~').contains(&next) {
                                break;
                            }
                        }
                    }
                    _ => {
                        while let Some(next) = chars.next() {
                            if ('@'..='~').contains(&next) {
                                break;
                            }
                        }
                    }
                }
                continue;
            }

            if ch.is_control() && ch != '\n' && ch != '\r' && ch != '\t' {
                continue;
            }

            output.push(ch);
        }

        output.replace('\r', "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_detection() {
        assert_eq!(TranslationEngine::detect_language("你好世界"), Language::Chinese);
        assert_eq!(TranslationEngine::detect_language("Hello World"), Language::English);
        assert_eq!(TranslationEngine::detect_language("Hello 世界"), Language::Chinese);
        assert_eq!(TranslationEngine::detect_language(""), Language::Auto);
    }

    #[test]
    fn test_extract_final_response() {
        let raw = "\u{1b}[2K\u{1b}[1GHello, world.\n";
        assert_eq!(TranslationEngine::extract_final_response(raw), "Hello, world.");
    }

    #[test]
    fn test_context_memory_add_and_find() {
        let mut ctx = TranslationContextMemory::new(10);

        ctx.add("Hello world", "你好世界");
        ctx.add("Good morning everyone", "大家早上好");
        ctx.add("Hello again", "又见面了");

        // "Hello world" should match "Hello again" due to token overlap
        let similar = ctx.find_similar("Hello there", 0.3, 3);
        assert!(!similar.is_empty());

        // All matches should contain "hello" token
        for r in &similar {
            assert!(r.source.to_lowercase().contains("hello"));
        }
    }

    #[test]
    fn test_context_memory_lru_trim() {
        let mut ctx = TranslationContextMemory::new(3);

        ctx.add("one", "一");
        ctx.add("two", "二");
        ctx.add("three", "三");
        ctx.add("four", "四");

        // Should only have 3 entries (oldest dropped)
        assert_eq!(ctx.recent.len(), 3);
        assert_eq!(ctx.recent[0].source, "two");
        assert_eq!(ctx.recent[2].source, "four");
    }

    #[test]
    fn test_context_memory_prompt_empty() {
        let ctx = TranslationContextMemory::new(10);
        assert_eq!(ctx.get_prompt_context("something new"), "");
    }

    #[test]
    fn test_context_memory_prompt_with_matches() {
        let mut ctx = TranslationContextMemory::new(10);
        ctx.add("user interface", "用户界面");
        ctx.add("error handling", "错误处理");

        let prompt = ctx.get_prompt_context("user input");
        assert!(prompt.contains("参考翻译"));
        assert!(prompt.contains("用户界面"));
    }

    #[test]
    fn test_jaccard_similarity() {
        let a: HashSet<String> = ["hello", "world", "test"]
            .iter().map(|s| s.to_string()).collect();
        let b: HashSet<String> = ["hello", "world", "foo"]
            .iter().map(|s| s.to_string()).collect();
        let sim = TranslationContextMemory::jaccard(&a, &b);
        // 2 intersection, 4 union -> 0.5
        assert!((sim - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_jaccard_no_overlap() {
        let a: HashSet<String> = ["apple", "banana"]
            .iter().map(|s| s.to_string()).collect();
        let b: HashSet<String> = ["cat", "dog"]
            .iter().map(|s| s.to_string()).collect();
        let sim = TranslationContextMemory::jaccard(&a, &b);
        assert_eq!(sim, 0.0);
    }
}
