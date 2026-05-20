use crate::error::{Result, VispError};
#[cfg(not(target_os = "macos"))]
use crate::models::ModelLoader;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// ASR engine.
///
/// On macOS: uses system Speech framework via macos_speech.m
/// On other platforms: uses whisper-rs (whisper.cpp) for offline Whisper inference
pub struct AsrEngine {
    backend: AsrBackend,
}

#[derive(Debug, Clone)]
enum AsrBackend {
    #[cfg(target_os = "macos")]
    AppleSpeech,
    #[cfg(not(target_os = "macos"))]
    Whisper {
        context: whisper_rs::WhisperContext,
    },
    #[cfg(not(target_os = "macos"))]
    Unavailable { reason: String },
}

#[cfg(target_os = "macos")]
#[link(name = "visp_macos_speech", kind = "static")]
#[link(name = "Speech", kind = "framework")]
unsafe extern "C" {
    fn visp_check_speech_permission() -> c_int;
    fn visp_get_speech_permission_status() -> c_int;
    fn visp_request_speech_permission(error: *mut c_char, error_len: c_int) -> c_int;
    fn visp_transcribe_file(
        path: *const c_char,
        output: *mut c_char,
        output_len: c_int,
        error: *mut c_char,
        error_len: c_int,
    ) -> c_int;
}

impl AsrEngine {
    pub fn new() -> Result<Self> {
        tracing::info!("初始化 ASR 引擎...");

        #[cfg(target_os = "macos")]
        {
            tracing::info!("使用 macOS Speech 识别后端");
            return Ok(Self {
                backend: AsrBackend::AppleSpeech,
            });
        }

        #[cfg(not(target_os = "macos"))]
        {
            let model_loader = ModelLoader::new()?;
            let whisper_model = model_loader.get_asr_model_path();

            if !whisper_model.exists() {
                tracing::warn!("Whisper 模型不存在: {:?}", whisper_model);
                return Ok(Self {
                    backend: AsrBackend::Unavailable {
                        reason: "当前环境没有可用的 Whisper 模型文件。请在应用的设置中下载 Whisper 模型。".to_string(),
                    },
                });
            }

            model_loader.verify_model(&whisper_model)?;
            tracing::info!("加载 Whisper 模型: {:?}", whisper_model);

            let context_params = whisper_rs::WhisperContextParameters::default();
            let context = whisper_rs::WhisperContext::new_with_params(
                &whisper_model.to_string_lossy(),
                context_params,
            ).map_err(|e| VispError::AsrError(format!("Whisper 上下文加载失败: {}", e)))?;

            tracing::info!("Whisper 模型加载成功");
            Ok(Self {
                backend: AsrBackend::Whisper { context },
            })
        }
    }

    pub fn transcribe(&self, audio_data: &[f32], sample_rate: u32) -> Result<String> {
        tracing::info!(
            "开始语音识别，音频长度: {} 样本, 采样率: {} Hz",
            audio_data.len(),
            sample_rate
        );

        let processed_audio = self.preprocess_audio(audio_data, sample_rate)?;
        let wav_path = self.write_temp_wav(&processed_audio, 16_000)?;

        let transcription = match &self.backend {
            #[cfg(target_os = "macos")]
            AsrBackend::AppleSpeech => self.transcribe_with_apple_speech(&wav_path),
            #[cfg(not(target_os = "macos"))]
            AsrBackend::Whisper { context } => self.transcribe_with_whisper(context, &processed_audio),
            #[cfg(not(target_os = "macos"))]
            AsrBackend::Unavailable { reason } => {
                let _ = std::fs::remove_file(&wav_path);
                Err(VispError::AsrError(reason.clone()))
            }
        };

        let _ = std::fs::remove_file(&wav_path);
        transcription
    }

    fn preprocess_audio(&self, audio_data: &[f32], sample_rate: u32) -> Result<Vec<f32>> {
        let mut processed = audio_data.to_vec();

        if sample_rate != 16_000 {
            processed = Self::resample(&processed, sample_rate, 16_000)?;
        }

        Self::normalize(&mut processed);
        Ok(processed)
    }

    fn resample(audio: &[f32], from_rate: u32, to_rate: u32) -> Result<Vec<f32>> {
        if from_rate == to_rate {
            return Ok(audio.to_vec());
        }

        let ratio = to_rate as f64 / from_rate as f64;
        let new_len = (audio.len() as f64 * ratio) as usize;
        let mut resampled = Vec::with_capacity(new_len);

        for i in 0..new_len {
            let src_idx = i as f64 / ratio;
            let idx0 = src_idx.floor() as usize;
            let idx1 = (idx0 + 1).min(audio.len().saturating_sub(1));
            let frac = src_idx - idx0 as f64;
            let sample = audio[idx0] * (1.0 - frac as f32) + audio[idx1] * frac as f32;
            resampled.push(sample);
        }

        Ok(resampled)
    }

    fn normalize(audio: &mut [f32]) {
        if audio.is_empty() {
            return;
        }

        let max_abs = audio.iter().map(|&x| x.abs()).fold(0.0f32, f32::max);
        if max_abs > 0.0 {
            let scale = 1.0 / max_abs;
            for sample in audio.iter_mut() {
                *sample *= scale;
            }
        }
    }

    fn write_temp_wav(&self, audio_data: &[f32], sample_rate: u32) -> Result<PathBuf> {
        let mut path = std::env::temp_dir();
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| VispError::AsrError(format!("无法生成临时文件名: {}", e)))?
            .as_millis();
        path.push(format!("visp-recording-{}.wav", millis));

        let pcm_samples: Vec<i16> = audio_data
            .iter()
            .map(|sample| (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16)
            .collect();

        let data_len = (pcm_samples.len() * std::mem::size_of::<i16>()) as u32;
        let mut bytes = Vec::with_capacity(44 + data_len as usize);

        bytes.extend_from_slice(b"RIFF");
        bytes.extend_from_slice(&(36 + data_len).to_le_bytes());
        bytes.extend_from_slice(b"WAVE");
        bytes.extend_from_slice(b"fmt ");
        bytes.extend_from_slice(&16u32.to_le_bytes());
        bytes.extend_from_slice(&1u16.to_le_bytes());
        bytes.extend_from_slice(&1u16.to_le_bytes());
        bytes.extend_from_slice(&sample_rate.to_le_bytes());
        let byte_rate = sample_rate * 2;
        bytes.extend_from_slice(&byte_rate.to_le_bytes());
        bytes.extend_from_slice(&2u16.to_le_bytes());
        bytes.extend_from_slice(&16u16.to_le_bytes());
        bytes.extend_from_slice(b"data");
        bytes.extend_from_slice(&data_len.to_le_bytes());

        for sample in pcm_samples {
            bytes.extend_from_slice(&sample.to_le_bytes());
        }

        std::fs::write(&path, bytes)?;
        Ok(path)
    }

    #[cfg(target_os = "macos")]
    pub fn check_permissions() -> Result<bool> {
        Ok(unsafe { visp_check_speech_permission() == 1 })
    }

    #[cfg(not(target_os = "macos"))]
    pub fn check_permissions() -> Result<bool> {
        // whisper-rs does not require system permissions; it processes in-memory audio
        Ok(true)
    }

    #[cfg(target_os = "macos")]
    pub fn request_permissions() -> Result<bool> {
        let mut error = vec![0 as c_char; 1024];
        let success = unsafe {
            visp_request_speech_permission(error.as_mut_ptr(), error.len() as c_int)
        };

        if success == 1 {
            Ok(true)
        } else {
            Err(VispError::AsrError(Self::read_c_buffer(&error)))
        }
    }

    #[cfg(target_os = "macos")]
    pub fn permission_status_code() -> Result<i32> {
        Ok(unsafe { visp_get_speech_permission_status() as i32 })
    }

    #[cfg(not(target_os = "macos"))]
    pub fn permission_status_code() -> Result<i32> {
        Ok(1)
    }

    #[cfg(not(target_os = "macos"))]
    pub fn request_permissions() -> Result<bool> {
        Ok(true)
    }

    #[cfg(target_os = "macos")]
    fn transcribe_with_apple_speech(&self, wav_path: &PathBuf) -> Result<String> {
        let wav_path = wav_path
            .to_str()
            .ok_or_else(|| VispError::AsrError("音频路径包含无效字符".to_string()))?;
        let wav_path = CString::new(wav_path)
            .map_err(|e| VispError::AsrError(format!("无法准备音频路径: {}", e)))?;
        let mut output = vec![0 as c_char; 8192];
        let mut error = vec![0 as c_char; 1024];

        let success = unsafe {
            visp_transcribe_file(
                wav_path.as_ptr(),
                output.as_mut_ptr(),
                output.len() as c_int,
                error.as_mut_ptr(),
                error.len() as c_int,
            )
        };

        if success == 1 {
            Ok(Self::read_c_buffer(&output))
        } else {
            Err(VispError::AsrError(Self::read_c_buffer(&error)))
        }
    }

    #[cfg(not(target_os = "macos"))]
    fn transcribe_with_whisper(
        &self,
        context: &whisper_rs::WhisperContext,
        audio_data: &[f32],
    ) -> Result<String> {
        let mut state = context
            .create_state()
            .map_err(|e| VispError::AsrError(format!("Whisper 状态创建失败: {}", e)))?;

        let mut params = whisper_rs::FullParams::new(whisper_rs::SamplingStrategy::Greedy {
            best_of: 1,
        });
        params.set_language(None); // auto-detect
        params.set_n_threads(4);
        params.set_print_progress(false);

        state
            .full(params, audio_data)
            .map_err(|e| VispError::AsrError(format!("Whisper 推理失败: {}", e)))?;

        let num_segments = state.full_n_segments() as usize;

        let mut result = String::new();
        for i in 0..num_segments {
            let segment = state.get_segment(i as i32);
            if let Some(seg) = segment {
                if let Ok(text) = seg.to_str() {
                    if !result.is_empty() {
                        result.push(' ');
                    }
                    result.push_str(text);
                }
            }
        }

        if result.trim().is_empty() {
            return Err(VispError::AsrError("没有识别到有效文本".to_string()));
        }

        Ok(result.trim().to_string())
    }

    #[cfg(target_os = "macos")]
    fn read_c_buffer(buffer: &[c_char]) -> String {
        unsafe { CStr::from_ptr(buffer.as_ptr()) }
            .to_string_lossy()
            .trim()
            .to_string()
    }
}
