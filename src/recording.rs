use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;

const RECORDINGS_DIR: &str = "recordings";

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordingInfo {
    pub filename: String,
    pub size_bytes: u64,
    pub size_kb: f64,
    pub size_mb: f64,
    pub created_timestamp: u64,
    pub created_date: String,
}

/// Save audio recording data to file
pub fn save_recording(audio_data: &[u8], filename: &str) -> Result<String, String> {
    // Create recordings directory if it doesn't exist
    if !Path::new(RECORDINGS_DIR).exists() {
        std::fs::create_dir_all(RECORDINGS_DIR)
            .map_err(|e| format!("Failed to create recordings directory: {}", e))?;
    }

    // Create full file path
    let file_path = Path::new(RECORDINGS_DIR).join(filename);

    // Write audio data to file
    let mut file =
        File::create(&file_path).map_err(|e| format!("Failed to create audio file: {}", e))?;

    file.write_all(audio_data)
        .map_err(|e| format!("Failed to write audio data: {}", e))?;

    // Get file size for confirmation
    let file_size = audio_data.len();
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S UTC");

    println!(
        "Audio recording saved: {} ({} bytes) at {}",
        filename, file_size, timestamp
    );

    Ok(format!(
        "Audio recording saved successfully: {} ({:.2} KB) at {}",
        filename,
        file_size as f64 / 1024.0,
        timestamp
    ))
}

/// List all audio recordings
pub fn list_recordings() -> Result<Vec<String>, String> {
    if !Path::new(RECORDINGS_DIR).exists() {
        return Ok(vec![]);
    }

    let entries = std::fs::read_dir(RECORDINGS_DIR)
        .map_err(|e| format!("Failed to read recordings directory: {}", e))?;

    let mut recordings = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();

        if path.is_file() {
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                recordings.push(filename.to_string());
            }
        }
    }

    // Sort recordings by name (which includes timestamp)
    recordings.sort();
    recordings.reverse(); // Most recent first

    Ok(recordings)
}

/// Delete an audio recording
pub fn delete_recording(filename: &str) -> Result<String, String> {
    let file_path = Path::new(RECORDINGS_DIR).join(filename);

    if !file_path.exists() {
        return Err(format!("Recording file not found: {}", filename));
    }

    std::fs::remove_file(&file_path).map_err(|e| format!("Failed to delete recording: {}", e))?;

    Ok(format!("Recording deleted successfully: {}", filename))
}

/// Get metadata for a recording file
pub fn get_recording_metadata(filename: &str) -> Result<RecordingInfo, String> {
    let file_path = Path::new(RECORDINGS_DIR).join(filename);

    if !file_path.exists() {
        return Err(format!("Recording file not found: {}", filename));
    }

    let metadata =
        std::fs::metadata(&file_path).map_err(|e| format!("Failed to get file metadata: {}", e))?;

    let file_size = metadata.len();
    let created = metadata
        .created()
        .map_err(|e| format!("Failed to get creation time: {}", e))?;

    let created_timestamp = created
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| format!("Failed to convert timestamp: {}", e))?
        .as_secs();

    let created_date = chrono::DateTime::from_timestamp(created_timestamp as i64, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    Ok(RecordingInfo {
        filename: filename.to_string(),
        size_bytes: file_size,
        size_kb: file_size as f64 / 1024.0,
        size_mb: file_size as f64 / (1024.0 * 1024.0),
        created_timestamp,
        created_date,
    })
}
