# Visp 100% Completion Design Spec

## Context & Gap Analysis

Based on thorough code review (2026-04-04), Visp is ~85% complete with three key gaps:

### Gap 1: Whisper ASR only works on macOS
- macOS uses `macos_speech.m` (SFSpeechRecognizer) — works well
- Windows/Linux backend is `Unavailable { reason: "..." }`
- Cargo.toml declares `candle-*` ML libs but **none are imported or used**
- Impact: Voice mode unusable on non-macOS platforms

### Gap 2: Translation requires external Ollama CLI
- `TranslationEngine` only has `TranslationBackend::Ollama` — calls `Command::new("ollama")`
- On-device GGUF inference is not implemented
- Impact: Not truly offline/independent; users must install and run Ollama

### Gap 3: No translation history
- Zero implementation — no data model, no storage, no UI
- Impact: No way to review, reuse, or search past translations

## Tech Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Whisper backend | `whisper-rs` 0.14 (whisper.cpp FFI) | Most stable Rust binding to whisper.cpp, proven cross-platform, GGML format native |
| GGUF translation backend | Candle (`candle-core` + `candle-transformers`) | Already in deps, supports GGUF loading, pure Rust — no C/C++ compilation |
| Translation history DB | `rusqlite` + `chrono` | Single-file SQLite, zero-config, cross-platform, good Rust API |
| Build system | Same as current (cc for Objective-C) | Add whisper-rs via Cargo deps |

## Architecture

```
src-tauri/src/
├── asr.rs          → Modify: add #[cfg(not(target_os = "macos"))] whisper-rs backend
├── translation.rs  → Modify: add CandleInference backend alongside Ollama
├── history.rs      → NEW: SQLite store, CRUD for TranslationRecord
├── commands.rs     → Modify: +3 commands for history get/list/delete
├── lib.rs          → Modify: add mod history; wire commands
├── main.rs         → Modify: add history commands to invoke_handler
├── Cargo.toml      → Modify: add whisper-rs, rusqlite, remove unused candle-*
├── build.rs        → Modify: add whisper.cpp submodule build steps
```

## Frontend Changes

```
src/
├── components/
│   ├── HistoryPanel.tsx     → NEW: translation history list with search/delete
│   └── Settings.tsx         → Modify: add History tab with clear/export
├── App.tsx                   → Modify: open History panel from header
```

## Integration Points

- **ASR**: `asr.rs` transcribe() already called from commands.rs via `AsrEngine::transcribe()`
  - Whisper backend: init -> load model -> transcribe (audio: &[f32]) -> return String
- **Translation**: `translation.rs` translate() already called from commands.rs
  - Candle backend: init -> load GGUF -> tokenize -> generate -> return String
- **History**: New module, called from new commands.rs endpoints
  - commands.rs calls `HistoryStore::record()` after each translation
  - commands.rs calls `HistoryStore::list()` for UI display
- **State**: AppState may store `Arc<HistoryStore>` for cross-command access

## Error Handling

- Whisper load failure → fall back to "no ASR backend available" error on platforms without whisper-rs support
- Candle inference failure → fall back to Ollama if available, else error
- SQLite failure → return VispError::HistoryError, do not crash
- Graceful degradation: if Whisper fails but Ollama works, ASR fails but translation works, and vice versa
