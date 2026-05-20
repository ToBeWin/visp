use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Host, Stream, StreamConfig};
#[cfg(target_os = "macos")]
use std::os::raw::{c_char, c_int};
#[cfg(not(target_os = "macos"))]
use std::thread;
#[cfg(not(target_os = "macos"))]
use std::time::Duration;
use std::sync::{Arc, Mutex};
use crate::error::{Result, VispError};

#[cfg(target_os = "macos")]
#[link(name = "visp_macos_speech", kind = "static")]
#[link(name = "AVFoundation", kind = "framework")]
unsafe extern "C" {
    fn visp_check_microphone_permission() -> c_int;
    fn visp_get_microphone_permission_status() -> c_int;
    fn visp_request_microphone_permission(error: *mut c_char, error_len: c_int) -> c_int;
}

/// 音频捕获组件 - 使用 cpal 捕获麦克风音频
pub struct AudioCapture {
    _host: Host,
    device: Device,
    config: StreamConfig,
    buffer: Arc<Mutex<Vec<f32>>>,
    waveform: Arc<Mutex<Vec<f32>>>,
    stream: Option<Stream>,
    is_recording: Arc<Mutex<bool>>,
}

impl AudioCapture {
    /// 创建新的音频捕获实例
    #[allow(dead_code)]
    pub fn new() -> Result<Self> {
        Self::with_waveform_sink(Arc::new(Mutex::new(vec![0.0; 24])))
    }

    pub fn with_waveform_sink(waveform: Arc<Mutex<Vec<f32>>>) -> Result<Self> {
        tracing::info!("初始化音频捕获...");
        
        // 获取默认音频主机
        let host = cpal::default_host();
        
        // 获取默认输入设备（麦克风）
        let device = host.default_input_device()
            .ok_or_else(|| VispError::AudioCaptureError("未找到默认输入设备".to_string()))?;
        
        tracing::info!("使用音频设备: {}", device.name().unwrap_or_default());
        
        // 获取默认配置
        let config = device.default_input_config()
            .map_err(|e| VispError::AudioCaptureError(format!("无法获取设备配置: {}", e)))?;
        
        tracing::info!("音频配置: {:?}", config);
        
        // 转换为 StreamConfig
        let stream_config = config.into();
        
        Ok(Self {
            _host: host,
            device,
            config: stream_config,
            buffer: Arc::new(Mutex::new(Vec::new())),
            waveform,
            stream: None,
            is_recording: Arc::new(Mutex::new(false)),
        })
    }
    
    /// 开始录音
    pub fn start_recording(&mut self) -> Result<()> {
        tracing::info!("开始录音");
        
        // 检查是否已在录音
        let mut is_recording = self.is_recording.lock()
            .map_err(|e| VispError::AudioCaptureError(format!("锁定失败: {}", e)))?;
        
        if *is_recording {
            return Err(VispError::AudioCaptureError("已在录音中".to_string()));
        }
        
        // 清空缓冲区
        let mut buffer = self.buffer.lock()
            .map_err(|e| VispError::AudioCaptureError(format!("锁定缓冲区失败: {}", e)))?;
        buffer.clear();
        drop(buffer);
        
        // 创建音频流
        let buffer_clone = Arc::clone(&self.buffer);
        let waveform_clone = Arc::clone(&self.waveform);
        let sample_rate = self.config.sample_rate.0;
        
        let stream = self.device.build_input_stream(
            &self.config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                // 音频数据回调
                if let Ok(mut buffer) = buffer_clone.lock() {
                    // 转换为单声道（如果是立体声）
                    let mono_data = Self::to_mono(data);
                    buffer.extend_from_slice(&mono_data);
                    Self::update_waveform(&waveform_clone, &mono_data);
                    
                    // 限制最大录音时长（60 秒）
                    let max_samples = sample_rate as usize * 60;
                    if buffer.len() > max_samples {
                        let excess = buffer.len() - max_samples;
                        buffer.drain(0..excess);
                    }
                }
            },
            |err| {
                tracing::error!("音频流错误: {}", err);
            },
            None,
        ).map_err(|e| VispError::AudioCaptureError(format!("无法创建音频流: {}", e)))?;
        
        // 启动流
        stream.play()
            .map_err(|e| VispError::AudioCaptureError(format!("无法启动音频流: {}", e)))?;
        
        self.stream = Some(stream);
        *is_recording = true;
        
        tracing::info!("录音已启动");
        Ok(())
    }
    
    /// 停止录音并返回音频数据
    pub fn stop_recording(&mut self) -> Result<(Vec<f32>, u32)> {
        tracing::info!("停止录音");
        
        let mut is_recording = self.is_recording.lock()
            .map_err(|e| VispError::AudioCaptureError(format!("锁定失败: {}", e)))?;
        
        if !*is_recording {
            return Err(VispError::AudioCaptureError("未在录音中".to_string()));
        }
        
        // 停止流
        if let Some(stream) = self.stream.take() {
            drop(stream);
        }
        
        *is_recording = false;
        
        // 获取录音数据
        let buffer = self.buffer.lock()
            .map_err(|e| VispError::AudioCaptureError(format!("锁定缓冲区失败: {}", e)))?;
        
        let audio_data = buffer.clone();
        let sample_rate = self.config.sample_rate.0;
        
        // 检查录音时长
        let duration_secs = audio_data.len() as f32 / sample_rate as f32;
        tracing::info!("录音完成，时长: {:.2} 秒，样本数: {}", duration_secs, audio_data.len());
        
        // 检查最小时长（0.5 秒）
        if duration_secs < 0.5 {
            return Err(VispError::AudioCaptureError(
                format!("录音时长过短: {:.2} 秒（最少 0.5 秒）", duration_secs)
            ));
        }
        
        Ok((audio_data, sample_rate))
    }
    
    /// 检查麦克风权限
    pub fn check_permission() -> Result<bool> {
        tracing::info!("检查麦克风权限");

        #[cfg(target_os = "macos")]
        {
            return Ok(unsafe { visp_check_microphone_permission() == 1 });
        }

        #[cfg(not(target_os = "macos"))]
        {
            let host = cpal::default_host();
            match host.default_input_device() {
                Some(_) => {
                    tracing::info!("麦克风权限已授予");
                    Ok(true)
                }
                None => {
                    tracing::warn!("未找到麦克风设备或权限被拒绝");
                    Ok(false)
                }
            }
        }
    }

    /// 触发系统麦克风授权请求
    pub fn request_permission() -> Result<bool> {
        tracing::info!("请求麦克风权限");

        #[cfg(target_os = "macos")]
        {
            let mut error = vec![0 as c_char; 1024];
            let success = unsafe {
                visp_request_microphone_permission(error.as_mut_ptr(), error.len() as c_int)
            };

            if success == 1 {
                return Ok(true);
            }

            let message = unsafe { std::ffi::CStr::from_ptr(error.as_ptr()) }
                .to_string_lossy()
                .trim()
                .to_string();
            return Err(VispError::AudioCaptureError(if message.is_empty() {
                "麦克风权限未授权，请在系统设置中允许 Visp 使用麦克风".to_string()
            } else {
                message
            }));
        }

        #[cfg(not(target_os = "macos"))]
        {
            let mut capture = Self::new()?;
            capture.start_recording()?;
            thread::sleep(Duration::from_millis(200));
            capture.force_stop_recording()?;

            Ok(true)
        }
    }

    pub fn permission_status_code() -> Result<i32> {
        #[cfg(target_os = "macos")]
        {
            return Ok(unsafe { visp_get_microphone_permission_status() as i32 });
        }

        #[cfg(not(target_os = "macos"))]
        {
            Ok(if Self::check_permission()? { 1 } else { 0 })
        }
    }
    
    /// 转换为单声道
    fn to_mono(data: &[f32]) -> Vec<f32> {
        // 假设数据是交错的立体声 (L, R, L, R, ...)
        // 如果是单声道，直接返回
        if data.len() % 2 != 0 {
            return data.to_vec();
        }
        
        // 转换为单声道（取平均值）
        data.chunks(2)
            .map(|chunk| (chunk[0] + chunk.get(1).unwrap_or(&0.0)) / 2.0)
            .collect()
    }
    
    /// 获取当前录音状态
    pub fn is_recording(&self) -> bool {
        self.is_recording.lock()
            .map(|guard| *guard)
            .unwrap_or(false)
    }
    
    #[cfg(not(target_os = "macos"))]
    fn force_stop_recording(&mut self) -> Result<()> {
        let mut is_recording = self.is_recording.lock()
            .map_err(|e| VispError::AudioCaptureError(format!("锁定失败: {}", e)))?;

        if let Some(stream) = self.stream.take() {
            drop(stream);
        }

        *is_recording = false;
        Ok(())
    }
    fn update_waveform(waveform: &Arc<Mutex<Vec<f32>>>, samples: &[f32]) {
        if samples.is_empty() {
            return;
        }

        let buckets = 24usize;
        let step = (samples.len() / buckets).max(1);
        let mut next = Vec::with_capacity(buckets);

        for chunk in samples.chunks(step).take(buckets) {
            let peak = chunk
                .iter()
                .map(|sample| sample.abs())
                .fold(0.0f32, f32::max);
            next.push(peak);
        }

        while next.len() < buckets {
            next.push(0.0);
        }

        if let Ok(mut current) = waveform.lock() {
            *current = next;
        }
    }
}

impl Drop for AudioCapture {
    fn drop(&mut self) {
        if self.is_recording() {
            tracing::info!("AudioCapture 被销毁，停止录音");
            let _ = self.stop_recording();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[serial(ffisystem)]
    #[test]
    fn test_audio_capture_creation() {
        let result = AudioCapture::new();
        // 在没有音频设备的环境中可能失败
        if result.is_ok() {
            assert!(result.is_ok());
        }
    }

    #[serial(ffisystem)]
    #[test]
    fn test_permission_check() {
        let result = AudioCapture::check_permission();
        assert!(result.is_ok());
    }

    #[test]
    fn test_to_mono() {
        let stereo = vec![1.0, 2.0, 3.0, 4.0];
        let mono = AudioCapture::to_mono(&stereo);
        assert_eq!(mono, vec![1.5, 3.5]);
    }
}
