# Visp

Visp is a lightweight desktop translation assistant.

The goal is to make translation feel like a system-level workflow: trigger a shortcut, speak or select text, and get the translated result back into the app you are already using.

## Current Status

Visp is under active development. The macOS build is the primary target today.

What currently works:

- Global shortcuts for voice and text translation.
- Voice translation flow on macOS using the system Speech framework.
- Toggle-style voice recording: press once to start, press again to stop.
- A separate floating voice HUD near the bottom of the screen.
- Text translation from selected text with preview and confirm-before-replace.
- Translation history.
- Runtime diagnostics for microphone, speech recognition, translation backend, and model availability.
- Translation via local GGUF models through Candle when compatible files are present.
- Translation fallback through local Ollama when a supported model is available.

What is not finished yet:

- The non-macOS experience has not been polished to release quality.
- Whisper-based ASR is implemented for non-macOS builds, but still needs broader real-device testing.
- GGUF translation support depends on compatible Qwen-style model and tokenizer files; it is not a one-click model manager yet.
- Speech Recognition permission handling on macOS is still being hardened because macOS TCC registration can be sensitive to app identity and signing.
- There are no official binary releases yet.

## Platform Notes

### macOS

macOS is the main supported platform at the moment.

Voice recognition uses the macOS Speech framework. The app also needs microphone permission. If Visp does not appear under `System Settings -> Privacy & Security -> Speech Recognition`, return to the Visp window and trigger the voice permission flow from the app first.

### Windows and Linux

The codebase contains cross-platform pieces, including Whisper-based ASR and Tauri builds, but these platforms should be considered experimental until they receive the same level of testing as macOS.

## Shortcuts

Default shortcuts:

- Voice translation: `Command + Shift + V` on macOS, `Ctrl + Shift + V` elsewhere.
- Text translation: `Command + Shift + C` on macOS, `Ctrl + Shift + C` elsewhere.

The voice shortcut is toggle-based. Press once to start recording, then press again to stop and process the result.

## Translation Backends

Visp currently tries translation backends in this order:

1. Local Candle GGUF backend, when compatible model and tokenizer files are present.
2. Local Ollama backend, when Ollama is installed, running, and has a supported model.
3. Unavailable state with a diagnostic message.

Supported Ollama model names currently detected by the app:

- `translategemma:latest`
- `qwen3.5:0.8b`
- `qwen3.5:2b`
- `qwen3.5:9b`

Local GGUF file names currently checked by the app:

- `qwen3.5-0.8b-instruct-q4_k_m.gguf` with `qwen-tokenizer.json`
- `qwen2.5-0.5b-instruct-q4_k_m.gguf` with `tokenizer.json`

Use the in-app runtime diagnostics to see the exact model directory for your machine.

## Development

### Requirements

- Node 20+
- Rust stable
- Platform-specific Tauri dependencies
- macOS with Xcode command line tools for the macOS build
- Optional: Ollama for the local translation fallback

### Install

```bash
npm install
```

### Run in Development

```bash
npm run tauri:dev
```

For macOS permission testing, the bundled `.app` is often more reliable than the raw development binary:

```bash
npm run tauri:build
open src-tauri/target/release/bundle/macos/Visp.app
```

### Checks

```bash
npm run type-check
npm run build
cargo check --manifest-path src-tauri/Cargo.toml
```

### Build

```bash
npm run tauri:build
```

On macOS, the build script re-signs the generated app bundle with the configured bundle identifier so macOS privacy permissions attach to the expected app identity.

## Project Structure

- `src/` - React frontend.
- `src/components/` - Welcome flow, settings, translation preview, history, and voice HUD UI.
- `src-tauri/src/` - Rust backend for shortcuts, audio, ASR, translation, clipboard, keyboard simulation, diagnostics, and app windows.
- `src-tauri/src/macos_speech.m` - macOS Speech and microphone permission bridge.
- `src-tauri/src/inference/` - Local Candle inference backend.
- `scripts/` - Build and helper scripts.

## Roadmap

- Make macOS Speech permission onboarding fully reliable.
- Polish the floating voice HUD so it behaves like a small system input tool.
- Improve GGUF model setup and validation.
- Add clearer release packaging.
- Expand and verify Windows and Linux support.
- Clean up older planning documents and reduce repository noise.

## License

No license file has been added yet. Treat the project as not licensed for reuse until a license is explicitly added.
