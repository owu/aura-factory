# Aura Factory 技术实现文档

## 项目概述

Aura Factory 是一个基于 Rust 语言开发的免费开源桌面音视频转换工具，采用现代化的 GUI 框架 Slint 构建用户界面，利用 FFmpeg 作为底层音视频处理引擎。

### 核心特性
- 支持多种音视频格式转换
- 提供视频裁剪、分辨率调整、帧率设置等高级功能
- 跨平台支持（Windows/Linux）
- 实时进度显示和转换状态监控
- 版本过期检测机制

## 技术架构

### 整体架构图
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│    UI 层        │    │   业务逻辑层     │    │    FFmpeg 层    │
│  (Slint GUI)    │◄──►│  (Rust Core)    │◄──►│  (Sidecar)      │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### 技术栈
- **编程语言**: Rust 2024 Edition
- **GUI 框架**: Slint 1.14.1
- **音视频处理**: FFmpeg-sidecar 2.3.0
- **异步运行时**: Tokio 1.x
- **HTTP 客户端**: Reqwest
- **文件对话框**: RFD
- **序列化**: Serde
- **时间处理**: Chrono

## 项目结构

```
aura-factory/
├── src/                    # Rust 源代码
│   ├── main.rs            # 应用程序入口点
│   ├── consts.rs          # 常量定义
│   ├── conversion.rs      # 转换逻辑
│   ├── media.rs           # 媒体文件分析
│   └── utils/
│       ├── mod.rs         # 工具模块导出
│       └── time.rs        # 时间相关工具
├── ui/                    # 用户界面文件
│   ├── app.slint          # 主界面定义
│   ├── components/
│   │   └── components.slint # 自定义组件
│   └── statics/           # 静态资源
├── build.rs               # 构建脚本
├── build.ps1              # Windows 构建脚本
├── build.sh               # Linux 构建脚本
└── Cargo.toml            # 项目配置
```

## 核心模块详解

### 1. 主程序模块 (main.rs)

**主要功能**:
- 应用程序初始化和生命周期管理
- 用户界面事件处理
- 异步任务调度
- 版本过期检测

**关键实现**:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // FFmpeg 自动下载检查
    ffmpeg_sidecar::download::auto_download()?;
    
    // 创建 Slint 应用实例
    let app = App::new()?;
    
    // 网络时间获取（用于过期检测）
    let network_timestamp = Arc::new(AtomicI64::new(0));
    tokio::spawn(async move {
        let ts = utils::time::standard_time();
        network_timestamp.store(ts, Ordering::Relaxed);
    });
    
    // 文件选择回调
    app.on_select_file(move || {
        // 使用 RFD 打开文件对话框
        if let Some(path) = FileDialog::new().pick_file() {
            // 异步分析媒体文件信息
            tokio::spawn(async move {
                match media::probe_media_info(&path_str).await {
                    Ok((duration_str, info_str, _)) => {
                        // 更新 UI 显示
                        app.set_total_duration(duration_str.into());
                    }
                    Err(e) => { /* 错误处理 */ }
                }
            });
        }
    });
    
    app.run()?;
    Ok(())
}
```

### 2. 媒体分析模块 (media.rs)

**主要功能**:
- 使用 FFprobe 分析媒体文件元数据
- 提取视频/音频流信息
- 格式化显示信息

**数据结构**:
```rust
#[derive(Deserialize, Debug)]
pub struct ProbeResult {
    pub format: Format,
    pub streams: Vec<Stream>,
}

#[derive(Deserialize, Debug)]
pub struct Stream {
    pub codec_type: String,        // "video" 或 "audio"
    pub width: Option<i32>,        // 视频宽度
    pub height: Option<i32>,       // 视频高度
    pub r_frame_rate: Option<String>, // 帧率
    pub bit_rate: Option<String>,  // 比特率
    pub sample_rate: Option<String>, // 音频采样率
    pub codec_name: Option<String>, // 编解码器名称
}
```

**关键函数**:
- `probe_media_info()`: 分析媒体文件并返回格式化信息
- `get_probe_result()`: 执行 FFprobe 命令并解析 JSON 输出

### 3. 转换处理模块 (conversion.rs)

**主要功能**:
- 构建和执行 FFmpeg 转换命令
- 实时进度监控
- 转换取消支持

**核心转换流程**:
```rust
pub async fn run_conversion(
    input: String,          // 输入文件路径
    format: String,         // 输出格式
    start: String,          // 开始时间
    end: String,            // 结束时间
    v_res: String,          // 视频分辨率
    v_fps: String,          // 视频帧率
    v_bitrate: String,      // 视频比特率
    a_rate: String,         // 音频采样率
    a_bitrate: String,      // 音频比特率
    v_codec: String,        // 视频编解码器
    app_weak: Weak<App>,    // UI 弱引用
    cancel_flag: Arc<AtomicBool>, // 取消标志
) -> Result<()>
```

**进度监控机制**:
- 通过 FFmpeg 的进度事件获取转换进度
- 基于预期时长计算百分比
- 实时更新 UI 进度条和状态信息

### 4. 时间工具模块 (utils/time.rs)

**主要功能**:
- 网络时间同步（用于版本过期检测）
- 时间格式转换

**网络时间获取策略**:
- 预定义多个国内网站作为时间源
- 随机选择并尝试连接
- 失败时回退到本地系统时间

```rust
const WEB_URLS: &[WebSite] = &[
    WebSite { name: "百度", url: "http://www.baidu.com" },
    WebSite { name: "国家授时中心", url: "http://www.ntsc.ac.cn" },
    // ... 其他网站
];
```

## 用户界面设计

### 界面架构
采用 Slint 声明式 UI 框架，主要组件包括：

1. **主窗口 (App)**: 继承自 Window 的基础组件
2. **卡片组件 (Card)**: 可复用的内容容器
3. **标签按钮 (TabButton)**: 选项卡切换控件

### 界面布局
- **顶部**: Logo 和标题区域
- **中部**: 选项卡导航（常规/输出设置）
- **底部**: 文件选择、转换控制、进度显示

### 状态管理
通过 Slint 的 in-out property 机制管理应用状态：
- `selected-file`: 当前选中的文件路径
- `progress`: 转换进度 (0.0-1.0)
- `is-converting`: 是否正在转换
- `status-message`: 状态提示信息

## 构建和部署

### Windows 构建
```powershell
# 使用 build.ps1 脚本
./build.ps1
```
构建过程：
1. 清理之前的构建
2. 使用 cargo build 编译发布版本
3. 从 consts.rs 提取版本号
4. 重命名可执行文件

### Linux 构建
```bash
# 使用 build.sh 脚本
./build.sh
```
构建过程：
1. 创建标准的 Linux 包结构
2. 生成桌面文件和图标
3. 创建安装/卸载脚本

### 构建配置优化
在 Cargo.toml 中配置了优化的发布构建：
```toml
[profile.release]
opt-level = "z"      # 最小化代码大小
lto = true          # 链接时优化
codegen-units = 1   # 单代码生成单元
strip = true        # 去除调试符号
panic = "abort"     # 异常时直接终止
```

## 关键技术特性

### 1. 异步处理架构
- 使用 Tokio 运行时处理文件分析和转换任务
- UI 线程与后台任务分离，避免界面卡顿
- 基于回调的事件驱动模型

### 2. 错误处理机制
- 使用 Anyhow 库进行统一的错误处理
- 详细的错误信息反馈给用户
- 优雅的失败恢复机制

### 3. 资源管理
- 使用 Arc 和 Atomic 类型进行线程安全的数据共享
- 弱引用避免循环引用导致的内存泄漏
- 及时的资源释放和清理

### 4. 跨平台兼容性
- 条件编译处理平台差异（如 Windows 图标资源）
- 统一的文件路径处理
- 平台特定的构建脚本

## 性能优化

### 构建优化
- 启用 LTO（链接时优化）减小二进制大小
- 使用最小化优化级别 (-Oz)
- 去除调试符号和冗余代码

### 运行时优化
- 异步任务避免阻塞 UI 线程
- 合理的内存分配和释放
- 高效的字符串处理

## 安全考虑

### 输入验证
- 文件路径安全性检查
- 时间格式有效性验证
- 参数边界检查

### 资源访问
- 文件读写权限控制
- 网络请求超时设置
- 内存安全保证（Rust 所有权系统）

## 扩展性设计

### 模块化架构
- 清晰的模块边界和职责分离
- 易于添加新的文件格式支持
- 可扩展的转换参数配置

### 配置化设计
- 常量集中管理便于维护
- 构建过程参数化
- 界面元素可配置

## 已知限制和未来改进

### 当前限制
1. 依赖外部 FFmpeg 二进制文件
2. 转换参数配置相对基础
3. 批量处理支持有限

### 改进方向
1. 集成更多音视频处理功能
2. 添加预设配置模板
3. 支持批量文件处理
4. 增强错误恢复和重试机制

## 总结

Aura Factory 展示了现代 Rust 桌面应用开发的完整技术栈，结合了高性能的系统编程语言和现代化的 GUI 框架。项目采用了合理的架构设计，具有良好的可维护性和扩展性。通过详细的代码分析和模块化设计，为后续的功能扩展奠定了坚实的基础。