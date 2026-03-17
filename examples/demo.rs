use audio_core::{AudioMetadata, analyze_audio_metadata};

fn main() {
    println!("=== Audio-Core Feature Testing ===\n");

    println!("Test 1: Audio Metadata Analysis");
    println!("{}", "=".repeat(50));

    let audio_path = "data/example_1.wav";
    let srt_path = "data/example_1.srt";

    println!("Audio file: {}", audio_path);
    println!("SRT file:   {}", srt_path);
    println!();

    match analyze_audio_metadata(audio_path, srt_path) {
        Ok(metadata) => {
            print_audio_metadata(&metadata);
            println!();
        }
        Err(e) => {
            println!("Error: {}", e);
            println!(
                "   Make sure {} and {} exist in the data/ directory\n",
                audio_path, srt_path
            );
        }
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
