# 模型路径更新说明

## 更新内容

将应用标识符和模型存储路径从 `com.visp.ai-translator` 更新为 `visp.live-translator`。

## 修改的文件

### 1. 配置文件
- **src-tauri/tauri.conf.json**
  - `identifier`: `com.visp.translator` → `live.visp.translator`

### 2. Rust 代码
- **src-tauri/src/models.rs**
  - `ProjectDirs::from("com", "visp", "ai-translator")` → `ProjectDirs::from("live", "visp", "translator")`
  
- **src-tauri/src/config.rs**
  - `ProjectDirs::from("com", "visp", "ai-translator")` → `ProjectDirs::from("live", "visp", "translator")`

### 3. 下载脚本
- **scripts/download-models.sh**
  - macOS: `~/Library/Application Support/Visp/models` → `~/Library/Application Support/visp.live-translator/models`
  - Linux: `~/.local/share/Visp/models` → `~/.local/share/visp.live-translator/models`
  - Windows: `%APPDATA%\Visp\models` → `%APPDATA%\visp.live-translator\models`

- **scripts/download-models.ps1**
  - Windows: `%APPDATA%\Visp\models` → `%APPDATA%\visp.live-translator\models`

### 4. 文档
- **PROJECT-100-PERCENT.md**
- **MODEL-SETUP.md**

## 新的目录结构

### macOS
```
~/Library/Application Support/visp.live-translator/
├── models/
│   ├── whisper-large-v3-turbo-q4_0.bin
│   ├── whisper-tokenizer.json
│   ├── qwen3.5-0.8b-instruct-q4_k_m.gguf
│   └── qwen-tokenizer.json
└── config.json
```

### Linux
```
~/.local/share/visp.live-translator/
├── models/
│   ├── whisper-large-v3-turbo-q4_0.bin
│   ├── whisper-tokenizer.json
│   ├── qwen3.5-0.8b-instruct-q4_k_m.gguf
│   └── qwen-tokenizer.json
└── config.json
```

### Windows
```
%APPDATA%\visp.live-translator\
├── models\
│   ├── whisper-large-v3-turbo-q4_0.bin
│   ├── whisper-tokenizer.json
│   ├── qwen3.5-0.8b-instruct-q4_k_m.gguf
│   └── qwen-tokenizer.json
└── config.json
```

## 日志目录（未改变）

日志目录保持不变，仍然使用 "Visp" 名称：

- **macOS**: `~/Library/Logs/Visp/`
- **Linux**: `~/.local/share/visp/logs/`
- **Windows**: `%APPDATA%\Visp\logs\`

## 迁移说明

如果你之前已经下载了模型到旧路径，有两种选择：

### 选项 1: 移动现有模型（推荐）

**macOS/Linux:**
```bash
# 如果旧目录存在
OLD_DIR="$HOME/Library/Application Support/Visp/models"  # macOS
# OLD_DIR="$HOME/.local/share/Visp/models"  # Linux

NEW_DIR="$HOME/Library/Application Support/visp.live-translator/models"  # macOS
# NEW_DIR="$HOME/.local/share/visp.live-translator/models"  # Linux

if [ -d "$OLD_DIR" ]; then
    mkdir -p "$NEW_DIR"
    mv "$OLD_DIR"/* "$NEW_DIR/"
    echo "✅ 模型已迁移到新位置"
fi
```

**Windows (PowerShell):**
```powershell
$OLD_DIR = "$env:APPDATA\Visp\models"
$NEW_DIR = "$env:APPDATA\visp.live-translator\models"

if (Test-Path $OLD_DIR) {
    New-Item -ItemType Directory -Force -Path $NEW_DIR
    Move-Item "$OLD_DIR\*" $NEW_DIR -Force
    Write-Host "✅ 模型已迁移到新位置"
}
```

### 选项 2: 重新下载模型

直接运行下载脚本，模型会自动下载到新位置：

```bash
# macOS/Linux
./scripts/download-models.sh

# Windows
.\scripts\download-models.ps1
```

## 验证

运行应用后，检查模型是否在正确位置：

1. 启动应用：`npm run tauri dev`
2. 打开设置界面
3. 查看"模型目录"显示的路径
4. 应该显示：`~/Library/Application Support/visp.live-translator/models` (macOS)

## 技术细节

### ProjectDirs 参数说明

```rust
ProjectDirs::from("live", "visp", "translator")
```

- **qualifier**: `"live"` - 组织域名后缀
- **organization**: `"visp"` - 组织名称
- **application**: `"translator"` - 应用名称

这会生成以下路径：
- macOS: `~/Library/Application Support/visp.live-translator`
- Linux: `~/.local/share/visp.live-translator`
- Windows: `%APPDATA%\visp.live-translator`

### 为什么使用 visp.live-translator？

1. **域名一致性**: 与官网 `visp.live` 保持一致
2. **避免冲突**: 使用反向域名格式，避免与其他应用冲突
3. **专业性**: 符合应用标识符的最佳实践

## 编译验证

```bash
cargo check --manifest-path src-tauri/Cargo.toml
```

✅ 编译通过，无错误

---

**更新日期**: 2026-03-28  
**版本**: v0.1.0  
**影响**: 模型和配置文件存储路径
