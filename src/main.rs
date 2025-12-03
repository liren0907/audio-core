use audio_core::{
    AudioMetadata, RecordingInfo, analyze_audio_metadata, get_recording_metadata, list_recordings,
    save_recording,
};
use std::fs;

fn main() {
    println!("=== Audio-Core Feature Testing ===\n");

    // Test 1: Audio Metadata Analysis (using real files from data/)
    println!("🎵 Test 1: Audio Metadata Analysis");
    println!("{}", "=".repeat(50));

    let audio_path = "data/example_1.wav";
    let srt_path = "data/example_1.srt";

    match analyze_audio_metadata(audio_path, srt_path) {
        Ok(metadata) => {
            print_audio_metadata(&metadata);
            println!();
        }
        Err(e) => {
            println!("✗ Error: {}", e);
            println!(
                "   Make sure {} and {} exist in the data/ directory\n",
                audio_path, srt_path
            );
        }
    }

    // Test 2: Save Recording (copy from data/ to recordings/)
    println!("📝 Test 2: Saving Audio Recording");
    println!("{}", "=".repeat(50));

    match fs::read(audio_path) {
        Ok(audio_data) => {
            let filename = format!("saved_example_{}.wav", chrono::Utc::now().timestamp());
            match save_recording(&audio_data, &filename) {
                Ok(msg) => println!("✓ Success: {}\n", msg),
                Err(e) => println!("✗ Error: {}\n", e),
            }
        }
        Err(e) => {
            println!("✗ Error reading audio file: {}", e);
            println!("   Make sure {} exists\n", audio_path);
        }
    }

    // Test 3: List Recordings
    println!("📂 Test 3: Listing All Recordings");
    println!("{}", "=".repeat(50));

    match list_recordings() {
        Ok(recordings) => {
            if recordings.is_empty() {
                println!("No recordings found.");
            } else {
                println!("Found {} recording(s):", recordings.len());
                for (i, rec) in recordings.iter().enumerate() {
                    println!("  {}. {}", i + 1, rec);
                }
            }
            println!();
        }
        Err(e) => println!("✗ Error: {}\n", e),
    }

    // Test 4: Get Recording Metadata
    println!("📊 Test 4: Getting Recording Metadata");
    println!("{}", "=".repeat(50));

    // Get metadata for the first recording in the list
    match list_recordings() {
        Ok(recordings) => {
            if let Some(first_recording) = recordings.first() {
                match get_recording_metadata(first_recording) {
                    Ok(metadata) => {
                        print_recording_info(&metadata);
                        println!();
                    }
                    Err(e) => println!("✗ Error: {}\n", e),
                }
            } else {
                println!("No recordings available to analyze.\n");
            }
        }
        Err(e) => println!("✗ Error: {}\n", e),
    }

    println!("=== Testing Complete ===");
}

/// Pretty-print audio metadata
fn print_audio_metadata(metadata: &AudioMetadata) {
    println!("Audio Metadata Analysis:");
    println!(
        "  Duration:            {:.2} seconds ({:.2} minutes)",
        metadata.duration_seconds,
        metadata.duration_seconds / 60.0
    );
    println!("  Sample Rate:         {} Hz", metadata.sample_rate);
    println!("  Bitrate:             {} kbps", metadata.bitrate);
    println!("  Channels:            {}", metadata.channels);
    println!("\nSRT Subtitle Analysis:");
    println!("  Total Segments:      {}", metadata.srt_segments);
    println!(
        "  Speech Duration:     {:.2} seconds ({:.2} minutes)",
        metadata.srt_speech_duration,
        metadata.srt_speech_duration / 60.0
    );
    println!(
        "  Avg Segment Duration: {:.2} seconds",
        metadata.srt_avg_segment_duration
    );

    // Calculate speech density
    if metadata.duration_seconds > 0.0 {
        let speech_density = (metadata.srt_speech_duration / metadata.duration_seconds) * 100.0;
        println!("  Speech Density:      {:.2}%", speech_density);
    }
}

/// Pretty-print recording information
fn print_recording_info(info: &RecordingInfo) {
    println!("  Size (bytes):    {}", info.size_bytes);
    println!("  Size (KB):       {:.2} KB", info.size_kb);
    println!("  Size (MB):       {:.4} MB", info.size_mb);
    println!("  Created:         {}", info.created_date);
    println!("  Timestamp:       {}", info.created_timestamp);
}
