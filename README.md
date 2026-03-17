# Audio-Core

A Rust library for analyzing audio file metadata, parsing SRT subtitle timing data, and managing audio recordings on disk.

## Features

- **Audio Metadata Analysis** — Extract duration, sample rate, bitrate, and channel count from audio files (WAV and other formats supported by lofty)
- **SRT Subtitle Parsing** — Parse SRT files to calculate total speech duration, segment count, average segment length, and speech density
- **Recording Management** — Save, list, delete, and inspect audio recordings stored in a local `recordings/` directory

## Getting Started

```bash
# Build the library
cargo build

# Run the demo example (requires sample files in data/)
cargo run --example demo
```

The demo example uses `data/example_1.wav` and `data/example_1.srt` to demo audio metadata analysis.

## Usage

### Analyze Audio + SRT Metadata

```rust
use audio_core::analyze_audio_metadata;

let metadata = analyze_audio_metadata("path/to/audio.wav", "path/to/subtitles.srt").unwrap();

println!("Duration: {:.2}s", metadata.duration_seconds);
println!("Sample rate: {} Hz", metadata.sample_rate);
println!("SRT segments: {}", metadata.srt_segments);
println!("Speech duration: {:.2}s", metadata.srt_speech_duration);
```

### Manage Recordings

```rust
use audio_core::{save_recording, list_recordings, get_recording_metadata, delete_recording};

// Save audio data to recordings/
let audio_data: Vec<u8> = std::fs::read("audio.wav").unwrap();
save_recording(&audio_data, "my_recording.wav").unwrap();

// List all recordings (newest first)
let recordings = list_recordings().unwrap();

// Get file size and creation date
let info = get_recording_metadata("my_recording.wav").unwrap();
println!("{:.2} KB, created {}", info.size_kb, info.created_date);

// Delete a recording
delete_recording("my_recording.wav").unwrap();
```

## Project Structure

```
src/
├── lib.rs         # Public API re-exports
├── metadata.rs    # Audio and SRT analysis (AudioMetadata)
└── recording.rs   # Recording file management (RecordingInfo)
examples/
└── demo.rs        # Demo example
data/
├── example_1.wav  # Sample audio file
├── example_1.srt  # Sample subtitle file
└── ground_truth.json
```