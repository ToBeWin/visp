// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod state;
mod commands;
mod audio;
mod asr;
mod translation;
mod clipboard;
mod keyboard;
mod hotkey;
mod models;
mod downloader;
mod error;
mod logging;
mod history;
mod inference;

use state::AppState;
use commands::*;
use logging::LoggingManager;
use tauri::{LogicalSize, Manager, Size, WebviewUrl, WebviewWindowBuilder};

fn main() {
    // 初始化日志系统
    let logging_manager = LoggingManager::new()
        .expect("Failed to create logging manager");
    
    logging_manager.init("info")
        .expect("Failed to initialize logging");
    
    // 清理 30 天前的旧日志
    if let Err(e) = logging_manager.cleanup_old_logs(30) {
        tracing::warn!("Failed to cleanup old logs: {}", e);
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            // 初始化应用状态
            let app_state = AppState::new()?;
            app.manage(app_state);

            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_size(Size::Logical(LogicalSize::new(720.0, 540.0)));
                let _ = window.center();
            }

            if app.get_webview_window("island").is_none() {
                let island_window = WebviewWindowBuilder::new(
                    app,
                    "island",
                    WebviewUrl::App("index.html".into()),
                )
                .title("Visp Island")
                .inner_size(388.0, 108.0)
                .transparent(true)
                .decorations(false)
                .resizable(false)
                .visible(false)
                .focused(false)
                .always_on_top(true)
                .skip_taskbar(true)
                .shadow(false)
                .build()?;

                let _ = island_window.set_ignore_cursor_events(false);
            }
            
            // 初始化全局快捷键
            let state = app.state::<AppState>();
            let config = state.config.read()
                .map_err(|e| anyhow::anyhow!("Failed to read config: {}", e))?
                .clone();

            if let Err(e) = hotkey::init_global_shortcuts(
                &app.handle(),
                &config.hotkeys.voice_mode,
                &config.hotkeys.text_mode,
            ) {
                tracing::error!("Failed to initialize global shortcuts: {}", e);
            }
            
            tracing::info!("Visp AI Translator initialized");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_voice_mode,
            stop_voice_mode,
            start_text_mode,
            confirm_translation,
            cancel_translation,
            get_config,
            update_config,
            check_microphone_permission,
            check_speech_permission,
            request_permissions,
            open_permission_settings,
            show_main_window,
            get_waveform_data,
            translate_text,
            detect_language,
            check_models,
            get_runtime_readiness,
            download_models,
            get_history,
            search_history,
            export_history,
            delete_history_entry,
            clear_history,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
