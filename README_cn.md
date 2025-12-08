# aura-factory
一个免费开源的桌面音视频转换工具。

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

./build.ps1
 
```

### 3.2 Linux
```bash
./build.sh
 
```

## 4. 安装与卸载
- 4.1 Windows
  - 从 [发布页面](https://github.com/owu/aura-factory/releases) 下载最新版本。
  - 将 zip 文件解压到您选择的目录。
  - 运行 `AuraFactory.v0.0.1.x86_64-windows.exe`。

- 4.2 Linux 安装
  - 从 [发布页面](https://github.com/owu/aura-factory/releases) 下载最新版本。
```
mkdir ./AuraFactory.x86_64-linux  &&  tar  -xJf   ./AuraFactory.v0.0.1.x86_64-linux.tar.xz  -C  ./AuraFactory.x86_64-linux  &&  cd  ./AuraFactory.x86_64-linux  && sudo  make  install
```
- 4.3 Linux 卸载
```
cd  ./AuraFactory.x86_64-linux  &&  sudo make  uninstall && cd ../ &&  rm -rf  ./AuraFactory.x86_64-linux
```



## 5. 截图

![general](https://github.com/owu/aura-factory/raw/unstable/screenshot/general.jpg)

![output](https://github.com/owu/aura-factory/raw/unstable/screenshot/output.jpg)