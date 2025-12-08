use anyhow::Result;
use serde::Deserialize;

// Serde structs for ffprobe output
#[derive(Deserialize, Debug)]
pub struct ProbeResult {
    pub format: Format,
    #[serde(default)]
    pub streams: Vec<Stream>,
}

#[derive(Deserialize, Debug)]
pub struct Format {
    pub duration: String,
}

#[derive(Deserialize, Debug)]
pub struct Stream {
    pub codec_type: String, // "video" or "audio"
    #[serde(default)]
    pub width: Option<i32>,
    #[serde(default)]
    pub height: Option<i32>,
    #[serde(default)]
    pub r_frame_rate: Option<String>, // e.g. "30/1"
    #[serde(default)]
    pub bit_rate: Option<String>,
    #[serde(default)]
    pub sample_rate: Option<String>,
    #[serde(default)]
    pub codec_name: Option<String>,
}

// Returns (Duration String, Info String, Duration Secs)
pub async fn probe_media_info(input_path: &str) -> Result<(String, String, f64)> {
    let probe = get_probe_result(input_path).await?;
    let duration_secs: f64 = probe.format.duration.parse().unwrap_or(0.0);

    // Format duration
    let h = (duration_secs / 3600.0).floor();
    let m = ((duration_secs % 3600.0) / 60.0).floor();
    let s = (duration_secs % 60.0).floor();
    let duration_str = format!("{:02}:{:02}:{:02}", h, m, s);

    // Build info string
    let mut info_parts = Vec::new();

    // Find video stream
    if let Some(video) = probe.streams.iter().find(|s| s.codec_type == "video") {
        let res = if let (Some(w), Some(h)) = (video.width, video.height) {
            format!("{}x{}", w, h)
        } else {
            "Unknown Res".to_string()
        };

        let fps = if let Some(ref r) = video.r_frame_rate {
            // parse "30/1" or "30000/1001"
            let parts: Vec<&str> = r.split('/').collect();
            if parts.len() == 2 {
                let num: f64 = parts[0].parse().unwrap_or(0.0);
                let den: f64 = parts[1].parse().unwrap_or(1.0);
                if den > 0.0 {
                    format!("{:.2} fps", num / den)
                } else {
                    r.clone()
                }
            } else {
                r.clone()
            }
        } else {
            "".to_string()
        };

        let br = if let Some(ref b) = video.bit_rate {
            let val: f64 = b.parse().unwrap_or(0.0);
            format!("{}k", (val / 1000.0).round())
        } else {
            "".to_string()
        };

        let codec = video.codec_name.clone().unwrap_or_default();

        let v_info = format!("Video: {} {} {} {}", codec, res, fps, br);
        info_parts.push(v_info.trim().to_string());
    }

    // Find audio stream
    if let Some(audio) = probe.streams.iter().find(|s| s.codec_type == "audio") {
        let sr = if let Some(ref r) = audio.sample_rate {
            format!("{}Hz", r)
        } else {
            "".to_string()
        };
        let br = if let Some(ref b) = audio.bit_rate {
            let val: f64 = b.parse().unwrap_or(0.0);
            format!("{}k", (val / 1000.0).round())
        } else {
            "".to_string()
        };
        let a_info = format!("Audio: {} {}", sr, br);
        info_parts.push(a_info.trim().to_string());
    }

    let full_info = if info_parts.is_empty() {
        "No stream info".to_string()
    } else {
        info_parts.join(" | ")
    };

    Ok((duration_str, full_info, duration_secs))
}

pub async fn get_total_duration(input_path: &str) -> Result<f64> {
    // Wrapper to keep compatibility if needed, but we should use probe_media_info mainly.
    // Or just use extraction.
    let probe = get_probe_result(input_path).await?;
    let duration: f64 = probe.format.duration.parse().unwrap_or(0.0);
    Ok(duration)
}

pub async fn get_probe_result(input_path: &str) -> Result<ProbeResult> {
    use std::process::Command;

    let possible_paths = ["./ffprobe", "ffprobe", "./target/debug/ffprobe"];
    let mut output = None;

    for probe_cmd in possible_paths {
        let mut cmd = Command::new(probe_cmd);
        cmd.args(&[
            "-v",
            "quiet",
            "-print_format",
            "json",
            "-show_format",
            "-show_streams",
            input_path,
        ]);
        
        // On Windows, hide console window for subprocess
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
        }
        
        let res = cmd.output();

        if let Ok(out) = res {
            if out.status.success() {
                output = Some(out);
                break;
            }
        }
    }

    if let Some(out) = output {
        let stdout = String::from_utf8_lossy(&out.stdout);
        match serde_json::from_str::<ProbeResult>(&stdout) {
            Ok(probe) => {
                println!("[PROBE JSON] Parsed successfully.");
                Ok(probe)
            }
            Err(e) => {
                println!(
                    "[PROBE ERROR] Failed to parse JSON: {} \nOutput: {}",
                    e, stdout
                );
                Err(anyhow::anyhow!("JSON Parse Error"))
            }
        }
    } else {
        println!("[PROBE ERROR] Could not execute ffprobe.");
        Err(anyhow::anyhow!("ffprobe execution failed"))
    }
}

// Helper to parse "00:00:10.50" to seconds
pub fn parse_time_str(time_str: &str) -> f64 {
    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() == 3 {
        let h: f64 = parts[0].parse().unwrap_or(0.0);
        let m: f64 = parts[1].parse().unwrap_or(0.0);
        let s: f64 = parts[2].parse().unwrap_or(0.0);
        return h * 3600.0 + m * 60.0 + s;
    }
    0.0
}
