# aura-factory
Aura Factory 是一个基于 Rust 语言开发的免费开源桌面音视频转换工具，采用现代化的 GUI 框架 Slint 构建用户界面，利用 FFmpeg 作为底层音视频处理引擎。

Aura Factory 展示了现代 Rust 桌面应用开发的完整技术栈，结合了高性能的系统编程语言和现代化的 GUI 框架。

项目采用了合理的架构设计，具有良好的可维护性和扩展性。通过详细的代码分析和模块化设计，为后续的功能扩展奠定了坚实的基础。

---
[English](./README.md) | 中文 | [日本語](./README_jp.md)


## 1. 软件栈
- https://rust-lang.org
- https://slint.dev
- https://crates.io/crates/ffmpeg-sidecar

## 2. 调试
```
cargo run .
```

## 3. 构建
### 3.1 Windows
```powershell

./build/scripts/build.ps1
 
```

### 3.2 Linux
```bash
./build/scripts/build.sh
 
```

## 4. 安装与卸载
- 4.1 Windows
  - 从 [发布页面](https://github.com/owu/aura-factory/releases) 下载最新版本。
  - 将 zip 文件解压到您选择的目录。
  - 运行 `AuraFactory.v0.0.2.x86_64-windows.exe`。

- 4.2 Linux 安装
  - 从 [发布页面](https://github.com/owu/aura-factory/releases) 下载最新版本。
```
mkdir ./AuraFactory.x86_64-linux  &&  tar  -xJf   ./AuraFactory.v0.0.2.x86_64-linux.tar.xz  -C  ./AuraFactory.x86_64-linux  &&  cd  ./AuraFactory.x86_64-linux  && sudo  make  install
```
- 4.3 Linux 卸载
```
cd  ./AuraFactory.x86_64-linux  &&  sudo make  uninstall && cd ../ &&  rm -rf  ./AuraFactory.x86_64-linux
```



## 5. 截图
<p align="center">
  <img src="screenshot/general.png" width="48%" />
  <img src="screenshot/output.png" width="48%" />
</p>