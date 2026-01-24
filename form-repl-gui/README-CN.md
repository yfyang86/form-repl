# FORM REPL GUI

一个为 FORM 计算机代数系统构建的现代化图形用户界面，使用 [Tauri](https://tauri.app/) 开发。

![FORM REPL GUI](screenshot.png)

## 功能特性

- 🎨 **现代化深色主题** - 护眼设计，采用 Catppuccin 启发配色方案
- ✨ **语法高亮** - FORM 代码彩色显示，提高可读性
- 📝 **多行输入** - 编写复杂的 FORM 程序，支持正确格式
- 📜 **会话历史** - 使用 Ctrl+↑/↓ 浏览历史命令
- ⚡ **快速执行** - 使用 Tauri Rust 后端，原生性能
- 🖥️ **跨平台** - 支持 Windows、macOS 和 Linux

## 前置条件

1. **Rust** (1.77 或更新版本)
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Tauri CLI v2**
   ```sh
   cargo install tauri-cli --version "^2"
   ```

3. **FORM** - 必须有 FORM 可执行文件
   - 在 PATH 中，或
   - 设置 `FORM_PATH` 环境变量

### 平台特定依赖

**macOS:**
```sh
xcode-select --install
```

**Ubuntu/Debian:**
```sh
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

**Windows:**
- 安装 Visual Studio Build Tools 并包含 C++ 工作负载

## 安装

1. **克隆或解压项目：**
   ```sh
   cd form-repl-gui
   ```

2. **在开发模式下构建并运行：**
   ```sh
   cargo tauri dev
   ```
   注意：开发模式下会显示终端窗口。

3. **构建生产版本（无终端窗口）：**
   ```sh
   cargo tauri build
   ```

   构建后的应用程序位于 `src-tauri/target/release/bundle/`

### 运行无终端窗口

**macOS:**
```sh
# 构建应用包
cargo tauri build

# 从 Finder 运行或使用：
open src-tauri/target/release/bundle/macos/FORM\ REPL.app
```

**Windows:**
发布版本会自动隐藏控制台窗口。

**Linux:**
```sh
cargo tauri build
./src-tauri/target/release/form-repl-gui
```

## 使用方法

### 键盘快捷键

| 快捷键 | 功能 |
|--------|------|
| `Ctrl+Enter` | 执行代码 |
| `Ctrl+L` | 清除输出 |
| `Ctrl+↑` | 历史命令上一条 |
| `Ctrl+↓` | 历史命令下一条 |
| `Tab` | 插入 4 个空格 |

### 示例会话

1. 在输入区域输入 FORM 代码：
   ```
   Symbol x,y;
   Local E = (x+y)^3;
   Print;
   ```

2. 按 `Ctrl+Enter` 或点击 "Run"

3. 在输出区域查看结果：
   ```
   E =
      x^3 + 3*x^2*y + 3*x*y^2 + y^3;
   ```

### 设置 FORM 路径

如果 FORM 不在 PATH 中，请在运行前设置环境变量：

**macOS/Linux:**
```sh
export FORM_PATH=/path/to/form
cargo tauri dev
```

**Windows (PowerShell):**
```powershell
$env:FORM_PATH = "C:\path\to\form.exe"
cargo tauri dev
```

## 项目结构

```
form-repl-gui/
├── src/
│   └── index.html          # 前端 (HTML + CSS + JS)
├── src-tauri/
│   ├── src/
│   │   └── main.rs         # Rust 后端
│   ├── capabilities/
│   │   └── default.json    # Tauri v2 权限配置
│   ├── Cargo.toml          # Rust 依赖
│   ├── tauri.conf.json     # Tauri 配置
│   └── build.rs            # 构建脚本
└── README.md
```

## 自定义

### 更改主题

编辑 `src/index.html` 中的 CSS 变量：

```css
:root {
    --bg-primary: #1e1e2e;
    --bg-secondary: #313244;
    --accent-blue: #89b4fa;
    /* ... */
}
```

### 窗口大小

编辑 `src-tauri/tauri.conf.json`：

```json
"windows": [
  {
    "width": 900,
    "height": 700,
    "minWidth": 600,
    "minHeight": 400
  }
]
```

## 故障排除

### "找不到 FORM" 错误

1. 确保 FORM 已安装且可执行
2. 检查终端中 `form --version` 是否正常工作
3. 将 `FORM_PATH` 设置为 FORM 可执行文件的完整路径

### Linux 构建错误

安装所有必需依赖：
```sh
sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

### Windows 空白窗口

确保已安装 WebView2 运行时：
https://developer.microsoft.com/zh-cn/microsoft-edge/webview2/

## 许可证

Apache-2.0 许可证

## 相关链接

- [FORM 官方网站](http://www.nikhef.nl/~form)
- [Tauri 文档](https://tauri.app/v1/guides/)
- [FORM REPL CLI 版本](../form-repl-improved/)
