# 正元剪贴板图标生成指南

## 品牌信息
- **中文品牌名**: 正元
- **产品全名**: 正元剪贴板（ZhengYuan Clipboard）
- **英文缩写**: ZYBoard / ZYClip
- **Slogan**: 正本清源，粘贴有序

## 图标设计规范
- **背景色**: 深蓝色 #2C3E50
- **圆角半径**: 边长的22%
- **主图形颜色**: 白色
- **点缀色**: 金色圆点 #F7B731
- **风格**: 现代、极简、扁平、无阴影

## 图标尺寸清单（PNG格式）

需要生成以下PNG尺寸，不透明深蓝背景：

| 尺寸 | 文件名 | 用途 |
|------|--------|------|
| 16×16 | icon_16.png | 系统托盘 |
| 24×24 | icon_24.png | 菜单项 |
| 32×32 | icon_32.png | 任务栏快捷方式 |
| 48×48 | icon_48.png | 文件管理器 |
| 64×64 | icon_64.png | 高DPI小图标 |
| 128×128 | icon_128.png | 应用图标、安装程序 |
| 256×256 | icon_256.png | 高分辨率桌面图标（128的2倍） |
| 512×512 | icon_512.png | 应用商店、官网展示 |

### 重要说明
- 128×128 和 256×256 必须保持内容一致（256是128的两倍像素）
- 每个尺寸单独生成，不要直接缩放
- 小尺寸（≤48px）建议使用简化版图标（icon-simple.svg）

## 生成工具推荐

### 在线工具
1. **CloudConvert**: https://cloudconvert.com/svg-to-png
2. **Convertio**: https://convertio.co/zh/svg-png/
3. **TinyPNG**: https://tinypng.com/ （用于压缩PNG）

### 本地工具
1. **ImageMagick** (命令行):
   ```bash
   magick -background "#2C3E50" icon.svg -resize 16x16 icon_16.png
   magick -background "#2C3E50" icon.svg -resize 24x24 icon_24.png
   magick -background "#2C3E50" icon.svg -resize 32x32 icon_32.png
   magick -background "#2C3E50" icon.svg -resize 48x48 icon_48.png
   magick -background "#2C3E50" icon.svg -resize 64x64 icon_64.png
   magick -background "#2C3E50" icon.svg -resize 128x128 icon_128.png
   magick -background "#2C3E50" icon.svg -resize 256x256 icon_256.png
   magick -background "#2C3E50" icon.svg -resize 512x512 icon_512.png
   ```

2. **GIMP**: 打开SVG，导出为不同尺寸的PNG

3. **Inkscape**: 打开SVG，导出为PNG时设置尺寸

## 图标彩蛋
长按版本号会显示隐藏文本："正元 · 致凤元"

## 后续步骤
生成PNG图标后，需要转换为平台特定格式：
- Windows: .ico 格式（使用 ImageMagick: `magick icon_256.png icon.ico`）
- macOS: .icns 格式（使用 iconutil 工具）

## 商业化注意事项
⚠️ 请确保品牌"正元"已在中国商标网查询：
- 第9类（软件）
- 第42类（科技服务）

域名建议：
- zyboard.com
- zhengyuan.clip
- zhengyuanpaste.com
