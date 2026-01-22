# aura-factory
Aura Factory is a free and open-source desktop audio and video conversion tool developed in Rust. It uses the modern GUI framework Slint to build its user interface and utilizes FFmpeg as its underlying audio and video processing engine.

Aura Factory showcases a complete technology stack for modern Rust desktop application development, combining a high-performance system programming language with a modern GUI framework.

The project employs a sound architectural design, ensuring good maintainability and extensibility. Through detailed code analysis and modular design, it lays a solid foundation for future feature expansions.

---
English | [中文](./README_cn.md) | [日本語](./README_jp.md)


## 1. software stack 
- https://rust-lang.org
- https://slint.dev
- https://crates.io/crates/ffmpeg-sidecar

## 2. debug
```
cargo run .
```

## 3. build
### 3.1 windows
```powershell

./build.ps1
 
```

### 3.2 linux
```bash
./build.sh
 
```

## 4. install  && uninstall
- 4.1 windows
  - Download the latest release from the [releases page](https://github.com/owu/aura-factory/releases).
  - Extract the zip file to a directory of your choice.
  - Run `AuraFactory.v0.0.2.x86_64-windows.exe`.

- 4.2 linux install
  - Download the latest release from the [releases page](https://github.com/owu/aura-factory/releases).
```
mkdir ./AuraFactory.x86_64-linux  &&  tar  -xJf   ./AuraFactory.v0.0.2.x86_64-linux.tar.xz  -C  ./AuraFactory.x86_64-linux  &&  cd  ./AuraFactory.x86_64-linux  && sudo  make  install
```
- 4.3 linux uninstall
```
cd  ./AuraFactory.x86_64-linux  &&  sudo make  uninstall && cd ../ &&  rm -rf  ./AuraFactory.x86_64-linux
```



## 5. screenshot
<p align="center">
  <img src="screenshot/general.png" width="48%" />
  <img src="screenshot/output.png" width="48%" />
</p>