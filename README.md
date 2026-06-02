# 📋 正元剪贴板 (ZYBoard)

<div align="center">

**正本清源，粘贴有序**

基于 **Tauri 2.0 + Vue 3 + TypeScript + SQLite** 开发的现代化剪贴板管理工具

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue)](https://tauri.app/)
[![Vue](https://img.shields.io/badge/Vue-3.5-green)](https://vuejs.org/)

[English](#english) | [中文](#中文)

</div>

---

## ✨ 功能特性

### 核心功能
- ✅ **实时监听** - 自动监听剪贴板变化，支持文本和图片
- ✅ **历史记录** - 使用 SQLite 持久化存储，不丢失数据
- ✅ **快速搜索** - 支持关键词搜索历史记录
- ✅ **固定项目** - 重要内容可固定以便快速访问
- ✅ **内容分类** - 自动识别文本、URL、邮箱、图片等类型
- ✅ **快捷粘贴** - 支持快捷键直接粘贴到光标位置（J/K/L）
- ✅ **全局快捷键** - 自定义快捷键唤醒应用
- ✅ **系统托盘** - 最小化到系统托盘，不占用任务栏
- ✅ **美观界面** - 现代化渐变设计，毛玻璃效果
- ✅ **深色模式** - 支持深色/浅色主题切换
- ✅ **国际化** - 支持中英文切换

### 技术亮点
- 🚀 **Tauri 2.0** - 轻量级跨平台框架，体积小、性能高
- 💻 **Rust 后端** - 高性能剪贴板操作和数据处理
- 🎨 **Vue 3** - 响应式前端界面，组合式 API
- 📦 **TypeScript** - 类型安全保障
- 🗄️ **SQLite** - 本地数据持久化存储
- ⚡ **实时事件驱动** - 基于 Tauri IPC 的高效通信机制
- 🔧 **跨平台** - 支持 Windows、macOS、Linux

## 📸 界面预览

### 主界面
- 透明毛玻璃窗口设计
- 支持拖拽移动
- 可固定在最上层

### 功能演示
- 剪贴板实时监听
- 快捷键粘贴
- 图片预览
- 固定项目高亮显示

---

## 🏗️ 技术架构

### 前端 (Vue 3 + TypeScript)
```
src/
├── App.vue                    # 主应用组件
├── main.ts                    # 入口文件
├── styles.css                 # 全局样式
├── env.d.ts                   # TypeScript 类型声明
├── assets/                    # 静态资源
└── translations/              # 国际化文件
    ├── zh.json                # 中文翻译
    └── en.json                # 英文翻译
```

### 后端 (Rust + Tauri)
```
src-tauri/
├── src/
│   ├── main.rs                # Rust 入口文件
│   ├── lib.rs                 # Tauri 核心逻辑和命令定义
│   ├── database.rs            # SQLite 数据库操作
│   ├── cache.rs               # LRU 缓存实现
│   ├── utils.rs               # 工具函数（内容类型识别、去重）
│   ├── windows.rs             # Windows 平台特定实现
│   └── macos.rs               # macOS 平台特定实现
├── Cargo.toml                 # Rust 依赖配置
├── tauri.conf.json            # Tauri 应用配置
├── icons/                     # 应用图标
└── entitlements.plist         # macOS 权限配置
```

### 核心依赖

#### Rust (Cargo.toml)
```toml
[dependencies]
tauri = { version = "2.0", features = ["macos-private-api"] }
arboard = { version = "3.3", features = ["image-data"] }  # 跨平台剪贴板访问
tokio = { version = "1", features = ["full"] }            # 异步运行时
serde = { version = "1", features = ["derive"] }          # 序列化
rusqlite = { version = "0.30", features = ["bundled"] }   # SQLite 数据库
lru = "0.12"                                              # LRU 缓存
chrono = "0.4"                                            # 时间处理
image = "0.24"                                            # 图片处理
md-5 = "0.10"                                             # 内容哈希去重
```

#### Node.js (package.json)
```json
{
  "dependencies": {
    "@tauri-apps/api": "^2.11.0",
    "@tauri-apps/plugin-opener": "^2",
    "@tauri-apps/plugin-global-shortcut": "^2",
    "@tauri-apps/plugin-autostart": "^2",
    "vue": "^3.5.32"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2.11.1",
    "typescript": "~5.6.2",
    "vite": "^6.0.3"
  }
}
```

## 🚀 快速开始

### 环境要求
- **Node.js** >= 18.0.0
- **Rust** >= 1.70
- **npm** >= 9.0 或 **pnpm** >= 8.0

### 安装依赖
```bash
npm install
# 或
pnpm install
```

### 开发模式
```bash
npm run tauri dev
# 或
pnpm tauri dev
```

### 生产构建

#### Windows
```bash
npm run tauri build
```
构建产物位于：`src-tauri/target/release/bundle/nsis/`

#### macOS
```bash
npm run tauri build
```
构建产物位于：`src-tauri/target/release/bundle/dmg/` 和 `bundle/macos/`

> 💡 **macOS 签名提示**：项目包含 macOS 签名脚本（`sign-mac.sh`），如需代码签名请先配置开发者证书

#### Linux
```bash
npm run tauri build
```
构建产物位于：`src-tauri/target/release/bundle/appimage/`

### 下载预编译版本

访问 [Releases](https://github.com/DongJianZheng/zyboard/releases) 页面下载最新版本

---

## 🎮 快捷键

| 快捷键 | 功能 |
|--------|------|
| `自定义快捷键` | 显示/隐藏主窗口（默认：`Cmd+Shift+V` / `Ctrl+Shift+V`） |
| `J` | 粘贴第 1 项到光标位置 |
| `K` | 粘贴第 2 项到光标位置 |
| `L` | 粘贴第 3 项到光标位置 |
| `↑` / `↓` | 上下导航选中项 |
| `Enter` | 粘贴选中项 |
| `Delete` | 删除选中项 |
| `F` | 固定/取消固定选中项 |
| `Tab` | 切换到固定项目/历史记录 |
| `ESC` | 隐藏窗口 |

---

## 📡 IPC 通信

### Rust 后端命令

#### 剪贴板操作
```rust
#[tauri::command]
async fn read_clipboard() -> Result<ClipboardContent, String>

#[tauri::command]
async fn write_clipboard(content: ClipboardContent) -> Result<(), String>

#[tauri::command]
async fn start_clipboard_monitor(window: Window) -> Result<(), String>

#[tauri::command]
async fn stop_clipboard_monitor() -> Result<(), String>
```

#### 数据库操作
```rust
#[tauri::command]
async fn get_history(limit: Option<usize>) -> Result<Vec<ClipboardItem>, String>

#[tauri::command]
async fn add_to_history(item: ClipboardItem) -> Result<(), String>

#[tauri::command]
async fn delete_item(id: i64) -> Result<(), String>

#[tauri::command]
async fn clear_all_history() -> Result<(), String>

#[tauri::command]
async fn toggle_pin(id: i64) -> Result<bool, String>
```

#### 设置与配置
```rust
#[tauri::command]
async fn get_settings() -> Result<Settings, String>

#[tauri::command]
async fn save_settings(settings: Settings) -> Result<(), String>
```

### 前端调用示例
```typescript
import { invoke } from '@tauri-apps/api/core'

// 读取剪贴板
const content = await invoke<ClipboardContent>('read_clipboard')

// 写入剪贴板
await invoke('write_clipboard', {
  content: { text: 'Hello', image: null, type: 'text' }
})

// 获取历史记录
const history = await invoke<ClipboardItem[]>('get_history', { limit: 100 })

// 固定项目
await invoke('toggle_pin', { id: 123 })
```

### 事件监听
```typescript
import { listen } from '@tauri-apps/api/event'

// 监听剪贴板变化
const unlisten = await listen<ClipboardItem>('clipboard-change', (event) => {
  console.log('New clipboard item:', event.payload)
})

// 监听快捷键触发
await listen('shortcut-triggered', (event) => {
  console.log('Shortcut:', event.payload)
})
```

## 🎨 界面设计

### 布局结构
```
┌──────────────────────────────┐
│  🔍  正元剪贴板    [菜单]    │
├──────────────────────────────┤
│  [历史记录] [固定项目]        │
├──────────────────────────────┤
│  🔍 搜索剪贴板...            │
├──────────────────────────────┤
│  📌 文本  2026-06-02 10:30   │
│  Hello World                 │
│  [固定] [删除]               │
├──────────────────────────────┤
│  🖼️ 图片  2026-06-02 10:28   │
│  [预览缩略图]                │
│  [固定] [删除]               │
├──────────────────────────────┤
│  共 50 条历史记录 | 3 条固定  │
└──────────────────────────────┘
```

### 配色方案

#### 深色模式（默认）
- **背景**: 渐变紫色 (#667eea → #764ba2)
- **卡片**: rgba(255, 255, 255, 0.1) + 毛玻璃效果
- **文本**: 白色 (#ffffff)
- **强调**: 金黄色 (#ffd700) 用于固定项目
- **边框**: rgba(255, 255, 255, 0.2)

#### 浅色模式
- **背景**: 渐变蓝色 (#e0c3fc → #8ec5fc)
- **卡片**: rgba(255, 255, 255, 0.8)
- **文本**: 深灰色 (#333333)
- **强调**: 金黄色 (#ffd700) 用于固定项目
- **边框**: rgba(0, 0, 0, 0.1)

---

## 📊 数据结构

### ClipboardItem (剪贴板项)
```typescript
interface ClipboardItem {
  id: number              // 唯一标识（数据库自增）
  content: string         // 文本内容
  image: string | null    // Base64 图片数据
  type: ClipboardType     // 类型
  hash: string            // MD5 哈希（去重）
  timestamp: number       // 时间戳
  pinned: boolean         // 是否固定
}

enum ClipboardType {
  Text = 'text',           // 普通文本
  URL = 'url',             // URL 链接
  Email = 'email',         // 邮箱地址
  MultiLine = 'multiline', // 多行文本
  LongText = 'longtext',   // 长文本
  Image = 'image'          // 图片
}
```

### Settings (设置)
```typescript
interface Settings {
  darkMode: boolean           // 深色模式
  language: 'zh' | 'en'      // 语言
  shortcut: string            // 全局快捷键
  maxHistory: number          // 最大历史记录数
  autoStart: boolean          // 开机自启
  showPreview: boolean        // 显示预览
}
```

### 数据库表结构

#### clipboard_items 表
```sql
CREATE TABLE clipboard_items (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  content TEXT,
  image TEXT,
  type TEXT NOT NULL,
  hash TEXT NOT NULL UNIQUE,
  timestamp INTEGER NOT NULL,
  pinned BOOLEAN DEFAULT 0
);

CREATE INDEX idx_timestamp ON clipboard_items(timestamp DESC);
CREATE INDEX idx_pinned ON clipboard_items(pinned DESC, timestamp DESC);
```

#### settings 表
```sql
CREATE TABLE settings (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL
);
```

## 🔒 安全特性

- ✅ **内容验证** - 过滤空内容和无效数据
- ✅ **重复检测** - 使用 MD5 哈希避免重复记录
- ✅ **数量限制** - 可配置的最大历史记录数
- ✅ **权限控制** - 仅访问系统剪贴板，不读取其他数据
- ✅ **本地存储** - 数据存储在本地 SQLite 数据库，不上传云端

---

## 🌐 跨平台支持

### Windows
- ✅ WebView2 引擎（系统自带）
- ✅ Windows 10/11
- ✅ NSIS 安装包
- ✅ 支持 Win32 API 粘贴功能

### macOS
- ✅ WKWebView 引擎
- ✅ macOS 10.13+ (High Sierra 及以上)
- ✅ DMG 安装包和 App Bundle
- ✅ 需要辅助功能权限（剪贴板监听）
- ✅ 支持代码签名（需配置证书）

### Linux
- ✅ WebKitGTK 引擎
- ✅ 支持 Ubuntu、Debian、Fedora、Arch 等
- ✅ AppImage 和 deb 包

---

## 🗂️ 项目结构说明

```
clipboard-enhancer/
├── src/                          # 前端源码 (Vue 3 + TypeScript)
│   ├── App.vue                   # 主应用组件
│   ├── main.ts                   # 入口文件
│   ├── styles.css                # 全局样式
│   ├── translations/             # 国际化文件
│   └── assets/                   # 静态资源
├── src-tauri/                    # Rust 后端源码
│   ├── src/
│   │   ├── lib.rs                # Tauri 命令和核心逻辑
│   │   ├── database.rs           # SQLite 数据库操作
│   │   ├── cache.rs              # LRU 缓存
│   │   ├── utils.rs              # 工具函数
│   │   ├── windows.rs            # Windows 平台实现
│   │   └── macos.rs              # macOS 平台实现
│   ├── Cargo.toml                # Rust 依赖配置
│   ├── tauri.conf.json           # Tauri 应用配置
│   ├── icons/                    # 应用图标
│   ├── entitlements.plist        # macOS 权限配置
│   ├── Info.plist                # macOS 应用信息
│   └── gen/                      # 构建生成文件（gitignore）
├── dist/                         # 前端构建产物（gitignore）
├── package.json                  # Node.js 配置
├── tsconfig.json                 # TypeScript 配置
├── vite.config.ts                # Vite 构建配置
├── README.md                     # 项目说明
├── BRAND.md                      # 品牌规范
├── BUILD_MAC.md                  # macOS 构建指南
├── sign-mac.sh                   # macOS 签名脚本
├── unsign-mac.sh                 # macOS 取消签名脚本
└── adhoc-sign.sh                 # macOS 临时签名脚本
```

---

## 📝 开发指南

### 代码规范

#### Rust 代码
- 遵循 [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 进行代码检查
- 为公共 API 添加文档注释（`///`）
- 为复杂逻辑添加行内注释（`//`）

#### TypeScript 代码
- 遵循 [Vue 3 Style Guide](https://vuejs.org/style-guide/)
- 使用 `npm run lint` 进行代码检查
- 优先使用组合式 API (Composition API)
- 为组件添加 Props 和 Emits 类型定义

### 提交规范

遵循 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

```
feat: 添加新功能
fix: 修复 bug
docs: 文档更新
style: 代码格式调整
refactor: 代码重构
perf: 性能优化
test: 测试相关
chore: 构建/工具链相关
```

### 分支策略

```
main          - 主分支，稳定版本
develop       - 开发分支
feature/*     - 功能分支
bugfix/*      - 修复分支
```

---

## 🐛 故障排除

### Windows
**问题**: WebView2 未安装
**解决**: 从 [Microsoft 官网](https://developer.microsoft.com/microsoft-edge/webview2/) 下载安装

### macOS
**问题**: 剪贴板监听不工作
**解决**:
1. 打开"系统偏好设置" > "安全性与隐私" > "隐私"
2. 找到"辅助功能"，勾选"正元剪贴板"
3. 重启应用

**问题**: 构建失败，签名错误
**解决**: 运行 `./unsign-mac.sh` 取消签名，或配置正确的开发者证书

### Linux
**问题**: 缺少 WebKitGTK 依赖
**解决**:
```bash
# Ubuntu/Debian
sudo apt-get install libwebkit2gtk-4.0-dev

# Fedora
sudo dnf install webkit2gtk3-devel

# Arch
sudo pacman -S webkit2gtk
```

---

## 🤝 贡献指南

欢迎贡献代码、报告 Bug 或提出新功能建议！

### 如何贡献

1. **Fork** 本仓库
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'feat: Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 提交 **Pull Request**

### 开发前建议

- 先查看 [Issues](https://github.com/DongJianZheng/zyboard/issues) 确认未被实现
- 大改动前先 [Discussion](https://github.com/DongJianZheng/zyboard/discussions) 讨论
- 遵循现有代码风格
- 添加必要的测试和文档

---

## 📄 许可证

本项目采用 [MIT License](LICENSE) 开源协议。

```
MIT License

Copyright (c) 2026 董小正

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

## 🌟 致谢

感谢以下开源项目：

- [Tauri](https://tauri.app/) - 跨平台应用框架
- [Vue.js](https://vuejs.org/) - 渐进式前端框架
- [Rust](https://www.rust-lang.org/) - 系统编程语言
- [arboard](https://github.com/1Password/arboard) - 跨平台剪贴板库
- [SQLite](https://www.sqlite.org/) - 嵌入式数据库

---

## 📮 联系方式

- **作者**: 董小正
- **公众号**: 阿正的码农生活与技术思考
- **邮箱**: 1175639137@qq.com
- **GitHub**: [DongJianZheng/zyboard](https://github.com/DongJianZheng/zyboard)
- **GitCode**: [qq_34665176/zyboard](https://gitcode.com/qq_34665176/zyboard)
- **问题反馈**: [GitHub Issues](https://github.com/DongJianZheng/zyboard/issues)
- **讨论区**: [GitHub Discussions](https://github.com/DongJianZheng/zyboard/discussions)

---

<div align="center">

**如果这个项目对你有帮助，请给一个 ⭐️ Star！**

Made with ❤️ by ZhengYuan Tech

</div>

---

## English

# 📋 ZYBoard (ZhengYuan Clipboard)

<div align="center">

**Order from chaos**

A modern clipboard manager built with **Tauri 2.0 + Vue 3 + TypeScript + SQLite**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue)](https://tauri.app/)
[![Vue](https://img.shields.io/badge/Vue-3.5-green)](https://vuejs.org/)

</div>

---

## ✨ Features

### Core Features
- ✅ **Real-time Monitoring** - Auto-detect clipboard changes (text & images)
- ✅ **History** - Persistent storage with SQLite
- ✅ **Quick Search** - Keyword search in history
- ✅ **Pin Items** - Pin important items for quick access
- ✅ **Content Types** - Auto-detect text, URL, email, image types
- ✅ **Quick Paste** - Paste with hotkeys (J/K/L)
- ✅ **Global Hotkey** - Customizable global shortcut to show/hide
- ✅ **System Tray** - Minimize to system tray
- ✅ **Beautiful UI** - Modern gradient design with glassmorphism
- ✅ **Dark Mode** - Dark/Light theme support
- ✅ **i18n** - Support for Chinese and English

### Tech Highlights
- 🚀 **Tauri 2.0** - Lightweight, high-performance framework
- 💻 **Rust Backend** - High-performance clipboard operations
- 🎨 **Vue 3** - Reactive UI with Composition API
- 📦 **TypeScript** - Type safety
- 🗄️ **SQLite** - Local persistent storage
- ⚡ **Event-Driven** - Efficient Tauri IPC communication
- 🔧 **Cross-Platform** - Windows, macOS, Linux

---

## 🚀 Quick Start

### Requirements
- **Node.js** >= 18.0.0
- **Rust** >= 1.70
- **npm** >= 9.0 or **pnpm** >= 8.0

### Install
```bash
npm install
# or
pnpm install
```

### Development
```bash
npm run tauri dev
# or
pnpm tauri dev
```

### Build
```bash
npm run tauri build
```

---

## 🎮 Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Custom Shortcut` | Show/Hide window (default: `Cmd+Shift+V` / `Ctrl+Shift+V`) |
| `J` | Paste 1st item |
| `K` | Paste 2nd item |
| `L` | Paste 3rd item |
| `↑` / `↓` | Navigate items |
| `Enter` | Paste selected item |
| `Delete` | Delete selected item |
| `F` | Pin/Unpin selected item |
| `Tab` | Switch between pinned/history |
| `ESC` | Hide window |

---

## 📄 License

[MIT License](LICENSE)

---

<div align="center">

**If this project helps you, please give it a ⭐️ Star!**

Made with ❤️ by ZhengYuan Tech

</div>
