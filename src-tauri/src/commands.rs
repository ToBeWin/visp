use tauri::{Emitter, Manager, PhysicalPosition, PhysicalSize, Position, Size, State};
use crate::audio::AudioCapture;
use crate::config::AppConfig;
use crate::asr::AsrEngine;
use crate::clipboard::ClipboardManager;
use crate::downloader::ModelDownloader;
use crate::hotkey;
use crate::keyboard::KeyboardSimulator;
use crate::models::ModelLoader;
use crate::state::{AppState, Mode, VoiceSession};
use crate::translation::{Language, TranslationEngine};
use std::process::Command;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
#[tauri::command]
pub async fn check_models() -> std::result::Result<serde_json::Value, String> {
    let model_loader = ModelLoader::new()
        .map_err(|e| format!("Failed to create model loader: {}", e))?;
    
    let status = model_loader.check_models();
    
    Ok(serde_json::json!({
        "asr_available": status.asr_available,
        "translation_available": status.translation_available,
        "missing_files": status.missing_files,
        "models_dir": model_loader.get_models_dir().to_string_lossy(),
        "asr_summary": status.asr_summary,
        "asr_detail": status.asr_detail,
        "translation_summary": status.translation_summary,
        "translation_detail": status.translation_detail,
        "download_supported": status.download_supported,
    }))
}
#[tauri::command]
pub async fn get_runtime_readiness() -> std::result::Result<serde_json::Value, String> {
    let model_loader = ModelLoader::new()
        .map_err(|e| format!("Failed to create model loader: {}", e))?;
    let status = model_loader.check_models();
    let microphone_permission = AudioCapture::check_permission().unwrap_or(false);
    let speech_permission = AsrEngine::check_permissions().unwrap_or(false);
    let microphone_status = AudioCapture::permission_status_code().unwrap_or_default();
    let speech_status = AsrEngine::permission_status_code().unwrap_or_default();
    let ollama_cli_available = ModelLoader::ollama_cli_available();
    let ollama_service_reachable = ModelLoader::ollama_service_reachable();
    let ollama_model = ModelLoader::detect_ollama_translation_model();
    #[cfg(target_os = "macos")]
    let voice_ready = status.asr_available && microphone_permission && speech_permission;
    #[cfg(not(target_os = "macos"))]
    let voice_ready = status.asr_available && microphone_permission;
    let text_ready = status.translation_available;
    let mut actions = Vec::new();
    if !microphone_permission {
        actions.push("请在系统设置里开启麦克风权限。");
    }
    if !speech_permission {
        actions.push("请在系统设置里开启语音识别权限。");
    }
    if !ollama_cli_available {
        actions.push("请先安装 Ollama。");
    } else if !ollama_service_reachable {
        actions.push("请先启动 Ollama 服务。");
    } else if ollama_model.is_none() {
        actions.push("请先拉取 translategemma:latest 或 qwen3.5 模型。");
    }
    Ok(serde_json::json!({
        "platform": std::env::consts::OS,
        "voice_ready": voice_ready,
        "text_ready": text_ready,
        "microphone_permission": microphone_permission,
        "microphone_status": microphone_status,
        "speech_permission": speech_permission,
        "speech_status": speech_status,
        "asr_available": status.asr_available,
        "asr_summary": status.asr_summary,
        "asr_detail": status.asr_detail,
        "translation_available": status.translation_available,
        "translation_summary": status.translation_summary,
        "translation_detail": status.translation_detail,
        "models_dir": model_loader.get_models_dir().to_string_lossy(),
        "ollama_cli_available": ollama_cli_available,
        "ollama_service_reachable": ollama_service_reachable,
        "ollama_model": ollama_model,
        "recommended_actions": actions,
    }))
}
#[tauri::command]
pub async fn start_voice_mode(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> std::result::Result<(), String> {
    tracing::info!("启动语音模式");
    ensure_voice_permissions(&app)?;
    {
        let mut mode = state.current_mode.lock()
            .map_err(|e| format!("Failed to lock mode: {}", e))?;
        *mode = Some(Mode::Voice);
    }
    let mut session_slot = state.voice_session.lock()
        .map_err(|e| format!("Failed to lock voice session: {}", e))?;
    if session_slot.is_some() {
        return Err("语音录制已在进行中".to_string());
    }
    let (stop_tx, stop_rx) = mpsc::channel::<()>();
    let (result_tx, result_rx) = mpsc::channel::<std::result::Result<(Vec<f32>, u32), String>>();
    let waveform = Arc::new(Mutex::new(vec![0.0; 24]));
    let waveform_for_thread = Arc::clone(&waveform);
    thread::spawn(move || {
        let recording_result = (|| -> std::result::Result<(Vec<f32>, u32), String> {
            let mut audio_capture = AudioCapture::with_waveform_sink(waveform_for_thread)
                .map_err(|e| format!("Failed to create audio capture: {}", e))?;
            audio_capture.start_recording()
                .map_err(|e| format!("Failed to start recording: {}", e))?;
            stop_rx.recv()
                .map_err(|e| format!("Failed to receive stop signal: {}", e))?;
            audio_capture.stop_recording()
                .map_err(|e| format!("Failed to stop recording: {}", e))
        })();
        let _ = result_tx.send(recording_result);
    });
    *session_slot = Some(VoiceSession {
        stop_tx,
        result_rx,
        waveform,
    });
    show_island_window(
        &app,
        serde_json::json!({
            "visible": true,
            "phase": "recording",
            "title": "正在录音",
            "detail": "再次按快捷键结束并开始识别"
        }),
    )?;
    Ok(())
}
#[tauri::command]
pub async fn stop_voice_mode<R: tauri::Runtime>(
    state: State<'_, AppState>,
    app: tauri::AppHandle<R>,
) -> std::result::Result<(), String> {
    tracing::info!("停止语音模式");
    let (audio_data, sample_rate) = {
        let mut session_slot = state.voice_session.lock()
            .map_err(|e| format!("Failed to lock voice session: {}", e))?;
        let session = session_slot
            .take()
            .ok_or_else(|| "当前没有进行中的语音录制".to_string())?;
        session.stop_tx.send(())
            .map_err(|e| format!("Failed to send stop signal: {}", e))?;
        session.result_rx.recv_timeout(Duration::from_secs(2))
            .map_err(|e| format!("Timed out waiting for audio capture: {}", e))??
    };
    show_island_window(
        &app,
        serde_json::json!({
            "visible": true,
            "phase": "thinking",
            "title": "正在识别与翻译",
            "detail": "请稍候，结果会自动写回当前输入位置"
        }),
    )?;
    let app_clone = app.clone();
    thread::spawn(move || {
        if let Err(e) = process_voice_recording(app_clone, audio_data, sample_rate) {
            tracing::error!("处理录音错误: {}", e);
        }
    });
    Ok(())
}
fn process_voice_recording<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    audio_data: Vec<f32>,
    sample_rate: u32,
) -> std::result::Result<(), String> {
    tracing::info!("处理录音中...");
    let asr_engine = app.state::<AppState>().get_or_init_asr()?;
    let asr_engine = asr_engine.lock()
        .map_err(|e| format!("Failed to lock ASR engine: {}", e))?;
    let transcribed_text = match asr_engine.transcribe(&audio_data, sample_rate) {
        Ok(text) => text,
        Err(err) => {
            let _ = hide_island_window(&app);
            emit_error(&app, &err.to_string());
            clear_current_mode(&app)?;
            return Err(err.to_string());
        }
    };
    tracing::info!("识别结果: {}", transcribed_text);
    let translator = app.state::<AppState>().get_or_init_translator()?;
    let translator = translator.lock().map_err(|e| format!("Failed to lock translator: {}", e))?;
    let translated_text = match translator.translate(
        &transcribed_text,
        Language::Auto,
        Language::Auto,
    ) {
        Ok(text) => text,
        Err(err) => {
            let _ = hide_island_window(&app);
            emit_error(&app, &err.to_string());
            clear_current_mode(&app)?;
            return Err(err.to_string());
        }
    };
    tracing::info!("翻译结果: {}", translated_text);
    // Record voice translation to history
    if let Ok(store) = app.state::<AppState>().history_store.lock() {
        let _ = store.record(&transcribed_text, &translated_text, "auto", "auto", "voice");
    }
    thread::sleep(Duration::from_millis(200));
    run_on_main_thread(&app, {
        let translated_text = translated_text.clone();
        move || {
            let mut keyboard = KeyboardSimulator::new()
                .map_err(|e| format!("Failed to create keyboard simulator: {}", e))?;
            keyboard
                .type_text_fast(&translated_text)
                .map_err(|e| format!("Failed to type text: {}", e))
        }
    })?;
    tracing::info!("语音模式完成");
    hide_island_window(&app)?;
    clear_current_mode(&app)?;
    app.emit("voice-mode-complete", ())
        .map_err(|e| format!("Failed to emit event: {}", e))?;
    Ok(())
}
#[tauri::command]
pub async fn start_text_mode<R: tauri::Runtime>(
    state: State<'_, AppState>,
    app: tauri::AppHandle<R>,
) -> std::result::Result<String, String> {
    tracing::info!("启动文本模式");
    {
        let mut mode = state.current_mode.lock()
            .map_err(|e| format!("Failed to lock mode: {}", e))?;
        *mode = Some(Mode::Text);
    }
    let clipboard = ClipboardManager::new()
        .map_err(|e| format!("Failed to create clipboard manager: {}", e))?;
    clipboard.backup()
        .map_err(|e| format!("Failed to backup clipboard: {}", e))?;
    ClipboardManager::simulate_copy()
        .map_err(|e| format!("Failed to simulate copy: {}", e))?;
    thread::sleep(Duration::from_millis(150));
    let selected_text = clipboard.read_text()
        .map_err(|e| format!("Failed to read clipboard: {}", e))?;
    if selected_text.is_empty() {
        clipboard.restore()
            .map_err(|e| format!("Failed to restore clipboard: {}", e))?;
        clear_current_mode_from_state(&state)?;
        return Err("未选中任何文本".to_string());
    }
    tracing::info!("选中文本: {}", selected_text);
    let translator = state.get_or_init_translator()?;
    let translator = translator.lock().map_err(|e| format!("Failed to lock translator: {}", e))?;
    let translated_text = translator.translate(
        &selected_text,
        Language::Auto,
        Language::Auto,
    ).map_err(|e| format!("Translation failed: {}", e))?;
    tracing::info!("翻译结果: {}", translated_text);
    // Record text translation to history
    if let Ok(store) = state.history_store.lock() {
        let _ = store.record(&selected_text, &translated_text, "auto", "auto", "text");
    }
    app.emit("text-mode-preview", translated_text.clone())
        .map_err(|e| format!("Failed to emit event: {}", e))?;
    clipboard.restore()
        .map_err(|e| format!("Failed to restore clipboard: {}", e))?;
    Ok(translated_text)
}
#[tauri::command]
pub async fn confirm_translation(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
    translation_text: String,
) -> std::result::Result<(), String> {
    tracing::info!("确认翻译");
    
    // 1. 创建剪贴板管理器
    let clipboard = ClipboardManager::new()
        .map_err(|e| format!("Failed to create clipboard manager: {}", e))?;
    
    // 2. 备份当前剪贴板
    clipboard.backup()
        .map_err(|e| format!("Failed to backup clipboard: {}", e))?;
    
    // 3. 将翻译结果写入剪贴板
    clipboard.write_text(&translation_text)
        .map_err(|e| format!("Failed to write clipboard: {}", e))?;
    
    // 4. 等待一小段时间
    thread::sleep(Duration::from_millis(100));
    
    // 5. 模拟 Ctrl+V 粘贴
    run_on_main_thread(&app, move || {
        let mut keyboard = KeyboardSimulator::new()
            .map_err(|e| format!("Failed to create keyboard simulator: {}", e))?;
        keyboard.paste()
            .map_err(|e| format!("Failed to paste: {}", e))
    })?;
    
    // 6. 等待粘贴完成
    thread::sleep(Duration::from_millis(100));
    
    // 7. 恢复原剪贴板
    clipboard.restore()
        .map_err(|e| format!("Failed to restore clipboard: {}", e))?;
    
    // 8. 清除当前模式
    {
        let mut mode = state.current_mode.lock()
            .map_err(|e| format!("Failed to lock mode: {}", e))?;
        *mode = None;
    }
    
    tracing::info!("翻译已确认并替换");
    Ok(())
}
#[tauri::command]
pub async fn cancel_translation(state: State<'_, AppState>) -> std::result::Result<(), String> {
    tracing::info!("取消翻译");
    
    // 清除当前模式
    {
        let mut mode = state.current_mode.lock()
            .map_err(|e| format!("Failed to lock mode: {}", e))?;
        *mode = None;
    }
    
    tracing::info!("翻译已取消");
    Ok(())
}
#[tauri::command]
pub async fn get_config(state: State<'_, AppState>) -> std::result::Result<AppConfig, String> {
    let config = state.config.read()
        .map_err(|e| format!("Failed to read config: {}", e))?;
    Ok(config.clone())
}
#[tauri::command]
pub async fn update_config(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
    new_config: AppConfig,
) -> std::result::Result<(), String> {
    let mut config = state.config.write()
        .map_err(|e| format!("Failed to write config: {}", e))?;
    let old_voice = config.hotkeys.voice_mode.clone();
    let old_text = config.hotkeys.text_mode.clone();
    *config = new_config.clone();
    new_config.save().map_err(|e| e.to_string())?;
    if old_voice != new_config.hotkeys.voice_mode || old_text != new_config.hotkeys.text_mode {
        hotkey::apply_hotkey_config(
            &app,
            &new_config.hotkeys.voice_mode,
            &new_config.hotkeys.text_mode,
        ).map_err(|e| e.to_string())?;
    }
    tracing::info!("Configuration updated");
    Ok(())
}
#[tauri::command]
pub async fn check_microphone_permission() -> std::result::Result<bool, String> {
    AudioCapture::check_permission()
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn check_speech_permission() -> std::result::Result<bool, String> {
    AsrEngine::check_permissions()
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn request_permissions(
    app: tauri::AppHandle,
) -> std::result::Result<serde_json::Value, String> {
    let (microphone, speech) = request_missing_voice_permissions(&app)?;
    Ok(serde_json::json!({
        "microphone": microphone,
        "speech": speech,
        "granted": microphone && speech,
    }))
}
#[tauri::command]
pub async fn open_permission_settings(permission: Option<String>) -> std::result::Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        let url = match permission.as_deref() {
            Some("microphone") => "x-apple.systempreferences:com.apple.preference.security?Privacy_Microphone",
            Some("speech") => "x-apple.systempreferences:com.apple.preference.security?Privacy_SpeechRecognition",
            _ => "x-apple.systempreferences:com.apple.preference.security?Privacy",
        };
        Command::new("open")
            .arg(url)
            .status()
            .map_err(|e| format!("无法打开系统设置: {}", e))?;
        return Ok(());
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = permission;
        Err("当前平台暂未实现打开权限设置".to_string())
    }
}
#[tauri::command]
pub async fn show_main_window(app: tauri::AppHandle) -> std::result::Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
    Ok(())
}
#[tauri::command]
pub async fn get_waveform_data(state: State<'_, AppState>) -> std::result::Result<Vec<f32>, String> {
    let session_slot = state.voice_session.lock()
        .map_err(|e| format!("Failed to lock voice session: {}", e))?;
    if let Some(session) = session_slot.as_ref() {
        let waveform = session.waveform.lock()
            .map_err(|e| format!("Failed to lock waveform: {}", e))?;
        return Ok(waveform.clone());
    }
    Ok(vec![0.0; 24])
}
#[tauri::command]
pub async fn translate_text(
    state: State<'_, AppState>,
    text: String,
    _source_lang: String,
    _target_lang: String,
) -> std::result::Result<String, String> {
    tracing::info!("翻译文本: {}", text);
    let translator = state.get_or_init_translator()
        .map_err(|e| format!("Translation engine unavailable: {}", e))?;
    let translator = translator.lock().map_err(|e| format!("Failed to lock translator: {}", e))?;
    translator
        .translate(&text, Language::Auto, Language::Auto)
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn detect_language(text: String) -> std::result::Result<String, String> {
    let lang = TranslationEngine::detect_language(&text);
    
    let lang_str = match lang {
        Language::Chinese => "zh",
        Language::English => "en",
        Language::Auto => "auto",
    };
    
    Ok(lang_str.to_string())
}
#[tauri::command]
pub async fn download_models<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> std::result::Result<(), String> {
    tracing::info!("开始下载模型");
    let model_loader = ModelLoader::new()
        .map_err(|e| format!("Failed to create model loader: {}", e))?;
    let status = model_loader.check_models();
    if !status.download_supported {
        return Err("当前版本还没有接入应用内 GGUF/Whisper 推理后端，下载这些模型文件并不能直接让功能可用。请先按当前提示配置可用后端。".to_string());
    }
    let downloader = ModelDownloader::new(model_loader.get_models_dir().to_path_buf());
    let app_clone = app.clone();
    tokio::spawn(async move {
        if let Err(e) = downloader.download_all_models(app_clone).await {
            tracing::error!("下载模型失败: {}", e);
        }
    });
    Ok(())
}
#[tauri::command]
pub async fn get_history(
    state: State<'_, AppState>,
    limit: Option<usize>,
    offset: Option<usize>,
) -> std::result::Result<Vec<crate::history::TranslationRecord>, String> {
    let store = state.history_store.lock()
        .map_err(|e| format!("Failed to lock history store: {}", e))?;
    store.list(limit.unwrap_or(50), offset.unwrap_or(0))
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn delete_history_entry(
    state: State<'_, AppState>,
    id: String,
) -> std::result::Result<(), String> {
    let store = state.history_store.lock()
        .map_err(|e| format!("Failed to lock history store: {}", e))?;
    store.delete(&id).map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn clear_history(
    state: State<'_, AppState>,
) -> std::result::Result<(), String> {
    let store = state.history_store.lock()
        .map_err(|e| format!("Failed to lock history store: {}", e))?;
    store.clear().map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn search_history(
    state: State<'_, AppState>,
    query: String,
) -> std::result::Result<Vec<crate::history::TranslationRecord>, String> {
    let store = state.history_store.lock()
        .map_err(|e| format!("Failed to lock history store: {}", e))?;
    store.search(&query, 100)
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn export_history(
    state: State<'_, AppState>,
) -> std::result::Result<String, String> {
    let store = state.history_store.lock()
        .map_err(|e| format!("Failed to lock history store: {}", e))?;
    let entries = store.list(10000, 0)
        .map_err(|e| e.to_string())?;
    serde_json::to_string_pretty(&entries)
        .map_err(|e| format!("Failed to serialize history: {}", e))
}
fn emit_error<R: tauri::Runtime>(app: &tauri::AppHandle<R>, message: &str) {
    let _ = hide_island_window(app);
    let _ = app.emit("error", message.to_string());
}

fn ensure_voice_permissions<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
) -> std::result::Result<(), String> {
    let (microphone, speech) = request_missing_voice_permissions(app)?;
    if microphone && speech {
        return Ok(());
    }

    let speech_status = AsrEngine::permission_status_code().unwrap_or_default();
    let microphone_status = AudioCapture::permission_status_code().unwrap_or_default();

    Err(match (microphone, speech, microphone_status, speech_status) {
        (false, _, _, _) => "麦克风权限尚未授予，请在系统设置中允许 Visp 使用麦克风。".to_string(),
        (_, false, _, 0) => "语音识别权限还没有被系统登记。请先回到 Visp 前台，再点一次“授予权限”或再次触发语音快捷键。".to_string(),
        (_, false, _, _) => "语音识别权限未授权，请在系统设置的 Speech Recognition 中允许 Visp。".to_string(),
        _ => "语音翻译现在缺少系统权限，请完成授权后再试一次。".to_string(),
    })
}

fn request_missing_voice_permissions<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
) -> std::result::Result<(bool, bool), String> {
    let microphone_permission = AudioCapture::check_permission().map_err(|e| e.to_string())?;
    let speech_permission = AsrEngine::check_permissions().map_err(|e| e.to_string())?;

    if microphone_permission && speech_permission {
        return Ok((true, true));
    }

    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }

    run_on_main_thread(app, move || {
        let microphone = if microphone_permission {
            true
        } else {
            AudioCapture::request_permission().map_err(|e| e.to_string())?
        };

        let speech = if speech_permission {
            true
        } else {
            AsrEngine::request_permissions().map_err(|e| e.to_string())?
        };

        Ok((microphone, speech))
    })
}
fn show_island_window<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    payload: serde_json::Value,
) -> std::result::Result<(), String> {
    if let Some(window) = app.get_webview_window("island") {
        position_island_window(&window)?;
        let _ = window.show();
        let _ = window.emit("island:update", payload);
    }
    Ok(())
}
fn hide_island_window<R: tauri::Runtime>(app: &tauri::AppHandle<R>) -> std::result::Result<(), String> {
    if let Some(window) = app.get_webview_window("island") {
        let _ = window.emit("island:hide", ());
        let _ = window.hide();
    }
    Ok(())
}
fn position_island_window<R: tauri::Runtime>(
    window: &tauri::WebviewWindow<R>,
) -> std::result::Result<(), String> {
    let monitor = window
        .current_monitor()
        .map_err(|e| format!("Failed to read monitor: {}", e))?
        .or_else(|| {
            window
                .primary_monitor()
                .ok()
                .flatten()
        })
        .ok_or_else(|| "No monitor available".to_string())?;
    let scale_factor = monitor.scale_factor();
    let width = 388.0_f64;
    let height = 108.0_f64;
    let bottom_margin = 42.0_f64;
    let _ = window.set_size(Size::Physical(PhysicalSize::new(
        (width * scale_factor) as u32,
        (height * scale_factor) as u32,
    )));
    let x = monitor.position().x + ((monitor.size().width as f64 - width * scale_factor) / 2.0) as i32;
    let y = monitor.position().y
        + (monitor.size().height as f64 - height * scale_factor - bottom_margin * scale_factor) as i32;
    let _ = window.set_position(Position::Physical(PhysicalPosition::new(x, y)));
    Ok(())
}
fn run_on_main_thread<R, F, T>(
    app: &tauri::AppHandle<R>,
    operation: F,
) -> std::result::Result<T, String>
where
    R: tauri::Runtime,
    F: FnOnce() -> std::result::Result<T, String> + Send + 'static,
    T: Send + 'static,
{
    let (tx, rx) = mpsc::channel();
    app.run_on_main_thread(move || {
        let _ = tx.send(operation());
    })
    .map_err(|e| format!("Failed to schedule main-thread task: {}", e))?;
    rx.recv()
        .map_err(|e| format!("Failed to receive main-thread task result: {}", e))?
}
fn clear_current_mode<R: tauri::Runtime>(app: &tauri::AppHandle<R>) -> std::result::Result<(), String> {
    let state = app.state::<AppState>();
    clear_current_mode_from_app_state(state.inner())
}
fn clear_current_mode_from_state(state: &State<'_, AppState>) -> std::result::Result<(), String> {
    clear_current_mode_from_app_state(state.inner())
}
fn clear_current_mode_from_app_state(state: &AppState) -> std::result::Result<(), String> {
    let mut mode = state.current_mode.lock()
        .map_err(|e| format!("Failed to lock mode: {}", e))?;
    *mode = None;
    Ok(())
}
