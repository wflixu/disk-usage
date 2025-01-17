# DiskUsage

![](src-tauri/icons/128x128.png)

## 项目简介
DiskUsage 是一个用于分析磁盘占用的工具应用，支持 macOS 和 Windows 平台。该应用使用 Tauri、Vite、Vue 和 ECharts 开发，旨在提供一个直观的界面来查看和管理磁盘空间。

## 功能
- 支持 macOS 和 Windows 平台
- 直观的磁盘占用可视化
- 快速扫描磁盘
- 详细的文件和文件夹信息
- 多种图表展示方式

## 安装
请确保您的系统已经安装了 Node.js 和 Rust。

### 克隆仓库
```bash
git clone https://github.com/yourusername/disk-usage.git
cd disk-usage
```

### 安装依赖
```bash
npm install
```

### 运行应用
```bash
npm run tauri dev
```

## 使用
1. 打开应用后，选择要扫描的磁盘或文件夹。
2. 点击“扫描”按钮，等待扫描完成。
3. 查看扫描结果，使用图表和列表来分析磁盘占用情况。

## 贡献
欢迎贡献代码！请先 fork 本仓库，然后创建一个新的分支进行修改，提交 Pull Request。

## 许可证
本项目基于 MIT 许可证开源。

