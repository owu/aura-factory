use crate::{
    App,
    media::{get_total_duration, parse_time_str},
};
use anyhow::Result;
use ffmpeg_sidecar::{command::FfmpegCommand, event::FfmpegEvent};
use slint::Weak;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
}; // Assuming App is public or re-exported, needs crate root access

#[allow(clippy::too_many_arguments)]
pub async fn run_conversion(
    input: String,
    format: String,
    start: String,
    end: String,
    v_res: String,
    v_fps: String,
    v_bitrate: String,
    a_rate: String,
    a_bitrate: String,
    v_codec: String,
    app_weak: Weak<App>,
    cancel_flag: Arc<AtomicBool>,
) -> Result<()> {
    let input_path = std::path::Path::new(&input);
    let file_stem = input_path.file_stem().unwrap().to_str().unwrap();
    let parent_dir = input_path.parent().unwrap();

    let output_name = format!("{}_converted.{}", file_stem, format);
    let output_path = parent_dir.join(output_name);

    // Ensure we don't overwrite input if names collide
    if input_path == output_path {
        return Err(anyhow::anyhow!("Output path is same as input path"));
    }

    println!(
        "Starting conversion: {} -> {}",
        input,
        output_path.display()
    );

    // Run ffmpeg
    // Construct command for logging
    let mut log_cmd = format!("ffmpeg -i \"{}\"", input);
    if !start.is_empty() {
        log_cmd.push_str(&format!(" -ss {}", start));
    }
    if !end.is_empty() {
        log_cmd.push_str(&format!(" -to {}", end));
    }
    // Add advanced settings to log_cmd for debugging purposes
    if !v_res.is_empty() && v_res != "Source" {
        log_cmd.push_str(&format!(" -s {}", v_res));
    }
    if !v_fps.is_empty() && v_fps != "Source" {
        log_cmd.push_str(&format!(" -r {}", v_fps));
    }
    if !v_bitrate.is_empty() && v_bitrate != "Auto" {
        log_cmd.push_str(&format!(" -b:v {}", v_bitrate));
    }
    if !a_rate.is_empty() && a_rate != "Auto" {
        log_cmd.push_str(&format!(" -ar {}", a_rate));
    }
    if !a_bitrate.is_empty() && a_bitrate != "Auto" {
        log_cmd.push_str(&format!(" -b:a {}", a_bitrate));
    }

    log_cmd.push_str(&format!(" -y \"{}\"", output_path.to_str().unwrap()));
    println!("\n[CMD] Executing: {}\n", log_cmd);

    // Calculate expected duration based on cuts
    let mut expected_duration_secs: f64 = 0.0;

    // Parse user inputs to seconds
    let start_secs = parse_time_str(&start);
    let end_secs = parse_time_str(&end);

    if end_secs > start_secs {
        expected_duration_secs = end_secs - start_secs;
        println!(
            "[Logic] Expected duration from cut: {}s",
            expected_duration_secs
        );
    } else if !end.is_empty() {
        // Fallback: if only end is specified (implicitly start at 0)
        expected_duration_secs = end_secs;
        println!(
            "[Logic] Expected duration from end time: {}s",
            expected_duration_secs
        );
    }

    // [Fix] If we still don't have expected duration (no cuts), probe the file explicitly now.
    // Transcoding stream logging proved unreliable for catching 'Duration: ' line.
    if expected_duration_secs == 0.0 {
        if let Ok(d) = get_total_duration(&input).await {
            expected_duration_secs = d;
            // Adjust for start offset if any
            if start_secs > 0.0 {
                expected_duration_secs = (expected_duration_secs - start_secs).max(0.1);
            }
            println!(
                "[Logic] Probed total duration for progress: {}s",
                expected_duration_secs
            );
        }
    }

    tokio::task::spawn_blocking(move || -> Result<()> {
        let mut cmd_builder = FfmpegCommand::new();
        cmd_builder.input(&input);

        // --- Trimming ---
        if !start.is_empty() {
            cmd_builder.args(&["-ss", &start]);
        }
        if !end.is_empty() {
            cmd_builder.args(&["-to", &end]);
        }

        // --- Advanced Settings ---
        // Video Resolution (-s)
        if !v_res.is_empty() && v_res != "Source" {
            cmd_builder.args(&["-s", &v_res]);
        }
        // Video FPS (-r)
        if !v_fps.is_empty() && v_fps != "Source" {
            cmd_builder.args(&["-r", &v_fps]);
        }
        // Video Bitrate (-b:v)
        if !v_bitrate.is_empty() && v_bitrate != "Auto" {
            cmd_builder.args(&["-b:v", &v_bitrate]);
        }
        // Audio Sample Rate (-ar)
        if !a_rate.is_empty() && a_rate != "Auto" {
            cmd_builder.args(&["-ar", &a_rate]);
        }
        // Audio Bitrate (-b:a)
        if !a_bitrate.is_empty() && a_bitrate != "Auto" {
            cmd_builder.args(&["-b:a", &a_bitrate]);
        }

        // Video Codec (-c:v)
        if !v_codec.is_empty() && v_codec != "Auto" {
            let codec_arg = match v_codec.as_str() {
                "h264" => "libx264",
                "h265" => "libx265",
                "copy" => "copy",
                _ => &v_codec, // Pass through if uncertain or simple name
            };
            cmd_builder.args(&["-c:v", codec_arg]);
        }

        // Print command for debug
        println!(
            "[CMD Config] Resolution: {}, FPS: {}, VBitrate: {}, ARate: {}, ABitrate: {}",
            v_res, v_fps, v_bitrate, a_rate, a_bitrate
        );

        let mut cmd = cmd_builder
            .output(output_path.to_str().unwrap())
            .overwrite()
            .spawn()?;

        // Process events
        for event in cmd.iter()? {
            // Check cancellation
            if cancel_flag.load(Ordering::Relaxed) {
                println!("Cancellation requested. Stopping ffmpeg.");
                cmd.kill()?;
                // We'll break loop, and return special error or handle it.
                // If we kill, iter might end or error.
                return Err(anyhow::anyhow!("Conversion cancelled by user."));
            }

            match event {
                FfmpegEvent::Log(_level, msg) => {
                    // Debug output kept for verification
                    println!("[FFMPEG LOG] {}", msg);
                }
                FfmpegEvent::Progress(p) => {
                    if expected_duration_secs > 0.0 {
                        let current_timestamp = parse_time_str(&p.time);
                        let progress = (current_timestamp / expected_duration_secs).clamp(0.0, 1.0);

                        // Update UI
                        let _ = slint::invoke_from_event_loop({
                            let app_weak = app_weak.clone();
                            move || {
                                if let Some(app) = app_weak.upgrade() {
                                    app.set_progress(progress as f32);
                                    let percentage = (progress * 100.0) as i32;
                                    app.set_status_message(
                                        format!("Converting... {}%", percentage).into(),
                                    );
                                }
                            }
                        });
                    } else {
                        // Still 0? Just show converting.
                        let _ = slint::invoke_from_event_loop({
                            let app_weak = app_weak.clone();
                            move || {
                                if let Some(app) = app_weak.upgrade() {
                                    app.set_status_message(
                                        format!("Converting... (Time: {})", p.time).into(),
                                    );
                                }
                            }
                        });
                    }
                }
                _ => {}
            }
        }
        Ok(())
    })
    .await??;

    Ok(())
}
