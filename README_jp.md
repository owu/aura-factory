# aura-factory
無料でオープンソースのデスクトップ音声・動画変換ツールです。

---
[English](./README.md) | [中文](./README_cn.md) | 日本語

## 1. ソフトウェアスタック
- https://rust-lang.org
- https://slint.dev
- https://crates.io/crates/ffmpeg-sidecar

## 2. デバッグ
```
cargo run .
```

## 3. ビルド
### 3.1 Windows
```powershell

./build.ps1
 
```

### 3.2 Linux
```bash
./build.sh
 
```

## 4. インストールとアンインストール
- 4.1 Windows
  - [リリースページ](https://github.com/owu/aura-factory/releases) から最新リリースをダウンロードします。
  - 任意のディレクトリに zip ファイルを解凍します。
  - `AuraFactory.v0.0.1.x86_64-windows.exe` を実行します。

- 4.2 Linux インストール
  - [リリースページ](https://github.com/owu/aura-factory/releases) から最新リリースをダウンロードします。
```
mkdir ./AuraFactory.x86_64-linux  &&  tar  -xJf   ./AuraFactory.v0.0.1.x86_64-linux.tar.xz  -C  ./AuraFactory.x86_64-linux  &&  cd  ./AuraFactory.x86_64-linux  && sudo  make  install
```
- 4.3 Linux アンインストール
```
cd  ./AuraFactory.x86_64-linux  &&  sudo make  uninstall && cd ../ &&  rm -rf  ./AuraFactory.x86_64-linux
```



## 5. スクリーンショット

![general](https://github.com/owu/aura-factory/raw/unstable/screenshot/general.png)

![output](https://github.com/owu/aura-factory/raw/unstable/screenshot/output.png)