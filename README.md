# aura-factory
A free and open-source desktop audio and video conversion tool.

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
  - Run `AuraFactory.v0.0.1.x86_64-windows.exe`.

- 4.2 linux install
  - Download the latest release from the [releases page](https://github.com/owu/aura-factory/releases).
```
mkdir ./AuraFactory.x86_64-linux  &&  tar  -xJf   ./AuraFactory.v0.0.1.x86_64-linux.tar.xz  -C  ./AuraFactory.x86_64-linux  &&  cd  ./AuraFactory.x86_64-linux  && sudo  make  install
```
- 4.3 linux uninstall
```
cd  ./AuraFactory.x86_64-linux  &&  sudo make  uninstall && cd ../ &&  rm -rf  ./AuraFactory.x86_64-linux
```



## 5. screenshot

![general](https://github.com/owu/aura-factory/raw/unstable/screenshot/general.jpg)

![output](https://github.com/owu/aura-factory/raw/unstable/screenshot/output.jpg)
