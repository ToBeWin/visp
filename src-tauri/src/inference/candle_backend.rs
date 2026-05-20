use candle_core::Device;
use candle_transformers::generation::LogitsProcessor;
use candle_transformers::models::quantized_qwen2::ModelWeights;
use std::sync::Mutex;
use tokenizers::Tokenizer;

use crate::error::{Result, VispError};


const SEQ_LEN: usize = 8192; // conservative default for Qwen models

pub struct CandleInference {
    model: Mutex<ModelWeights>,
    tokenizer: Tokenizer,
    device: Device,
}

impl CandleInference {
    pub fn new(gguf_path: &std::path::Path, tokenizer_path: &std::path::Path) -> Result<Self> {
        tracing::info!("加载 Candle GGUF 模型: {:?}", gguf_path);

        let device = Device::Cpu;

        let mut reader = std::fs::File::open(gguf_path)
            .map_err(|e| VispError::ModelLoadError(format!("无法打开模型文件: {}", e)))?;

        let content = candle_core::quantized::gguf_file::Content::read(&mut reader)
            .map_err(|e| VispError::ModelLoadError(format!("无法解析 GGUF 模型: {}", e)))?;

        let model = ModelWeights::from_gguf(content, &mut reader, &device)
            .map_err(|e| VispError::ModelLoadError(format!("模型加载失败: {}", e)))?;

        let tokenizer = Tokenizer::from_file(tokenizer_path)
            .map_err(|e| VispError::ModelLoadError(format!("无法加载 tokenizer: {}", e)))?;

        tracing::info!("Candle GGUF 模型加载成功");
        Ok(Self {
            model: Mutex::new(model),
            tokenizer,
            device,
        })
    }

    pub fn generate(&self, prompt: &str, max_tokens: usize) -> Result<String> {
        let tokens = self.tokenizer.encode(prompt, false)
            .map_err(|e| VispError::TranslationError(format!("Tokenization 失败: {}", e)))?;

        let tokens = tokens.get_ids().to_vec();
        let input_len = tokens.len();
        let eos_token = self.tokenizer.token_to_id("</s>")
            .unwrap_or(2);

        let mut logits_processor = LogitsProcessor::new(299792458, None, None);
        let mut all_tokens = tokens.clone();

        let mut model_lock = self.model.lock()
            .map_err(|e| VispError::CandleError(format!("模型锁冲突: {}", e)))?;

        for _ in 0..max_tokens {
            let context_len = all_tokens.len().min(SEQ_LEN);
            let ctx_start = all_tokens.len() - context_len;
            let input_tensor = candle_core::Tensor::new(&all_tokens[ctx_start..], &self.device)
                .and_then(|t| t.unsqueeze(0))
                .map_err(|e| VispError::CandleError(format!("Tensor 创建失败: {}", e)))?;

            let pos = all_tokens.len() - context_len;
            let logits = model_lock.forward(&input_tensor, pos)
                .map_err(|e| VispError::CandleError(format!("推理失败: {}", e)))?;

            let logits = logits.squeeze(0)
                .map_err(|e| VispError::TranslationError(format!("squeeze 失败: {}", e)))?;

            let next_token = logits_processor.sample(&logits)
                .map_err(|e| VispError::TranslationError(format!("采样失败: {}", e)))?;
            all_tokens.push(next_token);

            if next_token == eos_token {
                break;
            }
        }

        drop(model_lock);

        let generated = &all_tokens[input_len..];
        let decoded = self.tokenizer.decode(generated, true)
            .map_err(|e| VispError::TranslationError(format!("解码失败: {}", e)))?;

        Ok(decoded.trim().to_string())
    }
}
