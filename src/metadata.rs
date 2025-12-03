use lofty::read_from_path;
use lofty::AudioFile;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioMetadata {
    pub duration_seconds: f64,
    pub sample_rate: u32,
    pub bitrate: u32,
    pub channels: u16,
    pub srt_segments: usize,
    pub srt_speech_duration: f64,
    pub srt_avg_segment_duration: f64,
}

/// Analyze audio file and SRT subtitle file to extract metadata
pub fn analyze_audio_metadata(audio_path: &str, srt_path: &str) -> Result<AudioMetadata, String> {
    // 1. Process Audio File
    let audio_path_obj = Path::new(audio_path);
    if !audio_path_obj.exists() {
        return Err(format!("Audio file not found: {}", audio_path));
    }

    let tagged_file =
        read_from_path(audio_path_obj).map_err(|e| format!("Failed to read audio file: {}", e))?;

    let properties = tagged_file.properties();
    let duration_seconds = properties.duration().as_secs_f64();
    let sample_rate = properties.sample_rate().unwrap_or(0);
    let bitrate = properties.audio_bitrate().unwrap_or(0);
    let channels = properties.channels().unwrap_or(0) as u16;

    // 2. Process SRT File
    let srt_path_obj = Path::new(srt_path);
    if !srt_path_obj.exists() {
        return Err(format!("SRT file not found: {}", srt_path));
    }

    let file = File::open(srt_path_obj).map_err(|e| format!("Failed to open SRT file: {}", e))?;
    let reader = BufReader::new(file);

    let mut segments = 0;
    let mut total_speech_duration = 0.0;

    // Regex to match SRT timestamp line: 00:00:00,000 --> 00:00:00,000
    let timestamp_re =
        Regex::new(r"(\d{2}):(\d{2}):(\d{2}),(\d{3})\s-->\s(\d{2}):(\d{2}):(\d{2}),(\d{3})")
            .map_err(|e| format!("Failed to compile regex: {}", e))?;

    for line in reader.lines() {
        let line = line.map_err(|e| format!("Failed to read line: {}", e))?;
        if let Some(caps) = timestamp_re.captures(&line) {
            segments += 1;

            let start_h: f64 = caps[1].parse().unwrap();
            let start_m: f64 = caps[2].parse().unwrap();
            let start_s: f64 = caps[3].parse().unwrap();
            let start_ms: f64 = caps[4].parse().unwrap();

            let end_h: f64 = caps[5].parse().unwrap();
            let end_m: f64 = caps[6].parse().unwrap();
            let end_s: f64 = caps[7].parse().unwrap();
            let end_ms: f64 = caps[8].parse().unwrap();

            let start_seconds = start_h * 3600.0 + start_m * 60.0 + start_s + start_ms / 1000.0;
            let end_seconds = end_h * 3600.0 + end_m * 60.0 + end_s + end_ms / 1000.0;

            total_speech_duration += end_seconds - start_seconds;
        }
    }

    let avg_segment_duration = if segments > 0 {
        total_speech_duration / segments as f64
    } else {
        0.0
    };

    Ok(AudioMetadata {
        duration_seconds,
        sample_rate,
        bitrate,
        channels,
        srt_segments: segments,
        srt_speech_duration: total_speech_duration,
        srt_avg_segment_duration: avg_segment_duration,
    })
}
