use anyhow::Result;
use rfd::FileDialog;
use slint::ComponentHandle;
use std::sync::{Arc, atomic::{AtomicBool, AtomicI64, Ordering}};

// Import modules
mod media;
mod conversion;
mod consts;
mod utils;

use media::probe_media_info;
use conversion::run_conversion;
use utils::time::standard_time;
use webbrowser;

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Ensure ffmpeg is installed
    println!("Checking ffmpeg installation...");
    if let Err(e) = ffmpeg_sidecar::download::auto_download() {
        eprintln!("Failed to auto-download ffmpeg: {}", e);
        // Fallback or just warn, maybe user has it in PATH but auto_download failed?
        // But usually os error 2 means it's not in PATH and sidecar didn't find it.
    }
    println!("FFmpeg check done.");

    let app = App::new()?;
    // Set Version
    app.set_app_version(consts::APP_VERSION.into());
    
    let app_weak = app.as_weak();

    // Open GitHub Link
    app.on_open_github_link(move || {
        let _ = webbrowser::open("https://github.com/owu/aura-factory");
    });

    // Time Check Logic
    // Store network timestamp in AtomicI64. Default 0 (not fetched yet) or valid timestamp.
    let network_timestamp = Arc::new(AtomicI64::new(0));
    
    // Spawn task to fetch time without blocking UI
    let net_ts_clone = network_timestamp.clone();
    tokio::spawn(async move {
        // Run standard_time (which is blocking http) in spawn_blocking if needed, 
        // but since we are in async main, and standard_time uses blocking reqwestClient...
        // Wait, standard_time uses blocking reqwest. We must use spawn_blocking.
        let ts = tokio::task::spawn_blocking(|| {
            standard_time()
        }).await.unwrap_or(0);
        
        net_ts_clone.store(ts, Ordering::Relaxed);
        println!("Network time fetched: {}", ts);
    });

    // Select File Callback
    let app_weak_select = app_weak.clone();
    app.on_select_file(move || {
        let app = app_weak_select.unwrap();

        // Open dialog
        if let Some(path) = FileDialog::new().pick_file() {
            let path_str = path.display().to_string();
            app.set_selected_file(path_str.clone().into());

            // Probe media info
            app.set_status_message("正在分析文件...".into());
            let app_weak_inner = app_weak_select.clone();
            tokio::spawn(async move {
                // Use extended probe that returns (duration_str, info_str, duration_secs)
                match probe_media_info(&path_str).await {
                    Ok((duration_str, info_str, _start_secs)) => {
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(app) = app_weak_inner.upgrade() {
                                app.set_total_duration(duration_str.into());
                                app.set_source_info(info_str.into());
                                app.set_status_message("文件已就绪".into());
                            }
                        });
                    }
                    Err(e) => {
                        println!("Probe failed: {}", e);
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(app) = app_weak_inner.upgrade() {
                                app.set_total_duration("未知".into());
                                app.set_source_info("无法读取信息".into());
                                app.set_status_message("分析失败".into());
                            }
                        });
                    }
                }
            });

            app.set_progress(0.0);
        }
    });

    // Thread-safe cancellation flag
    let cancel_flag = Arc::new(AtomicBool::new(false));

    // Cancel Callback
    let app_weak_cancel = app_weak.clone();
    let cancel_flag_btn = cancel_flag.clone();
    app.on_cancel_conversion(move || {
        // Set flag to true
        cancel_flag_btn.store(true, Ordering::Relaxed);
        if let Some(app) = app_weak_cancel.upgrade() {
            app.set_status_message("正在停止...".into());
        }
    });

    // Start Conversion Callback
    let app_weak_convert = app_weak.clone();
    let net_ts_check = network_timestamp.clone();
    
    app.on_start_conversion(move || {
        let app = app_weak_convert.unwrap();
        
        // CHECK EXPIRATION
        let current_net_ts = net_ts_check.load(Ordering::Relaxed);
        if current_net_ts > consts::EXPIRATION_TIMESTAMP {
            app.set_status_message(format!("❌ 版本已过期，请更新！(Code: {})", current_net_ts).into());
            return;
        }

        let input_file = app.get_selected_file().to_string();
        let format = app.get_output_format().to_string();
        let start_time = app.get_start_time().to_string();
        let end_time = app.get_end_time().to_string();

        // Advanced settings
        let v_res = app.get_video_resolution().to_string();
        let v_fps = app.get_video_fps().to_string();
        let v_bitrate = app.get_video_bitrate().to_string();
        let a_rate = app.get_audio_samplerate().to_string();
        let a_bitrate = app.get_audio_bitrate().to_string();
        let v_codec = app.get_video_codec().to_string();

        if input_file.is_empty() {
            app.set_status_message("⚠️ 请先选择一个文件".into());
            return;
        }

        app.set_is_converting(true);
        app.set_status_message("正在转换... 0%".into());
        app.set_progress(0.0);

        // Reset cancel flag
        cancel_flag.store(false, Ordering::Relaxed);

        let app_weak_inner = app_weak_convert.clone();
        let cancel_flag_inner = cancel_flag.clone();
        tokio::spawn(async move {
            let result = run_conversion(
                input_file,
                format,
                start_time,
                end_time,
                v_res,
                v_fps,
                v_bitrate,
                a_rate,
                a_bitrate,
                v_codec,
                app_weak_inner.clone(),
                cancel_flag_inner,
            )
            .await;

            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = app_weak_inner.upgrade() {
                    app.set_is_converting(false);
                    match result {
                        Ok(_) => {
                            app.set_status_message("✅ 转换成功完成！".into());
                            app.set_progress(1.0);
                        }
                        Err(e) => {
                            app.set_status_message(format!("❌ 错误: {}", e).into());
                        }
                    }
                }
            });
        });
    });

    app.run()?;
    Ok(())
}
