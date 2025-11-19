// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::{engine::general_purpose, Engine as _};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
enum FileType {
    Pdf,
    Png,
    Zplii,
    Epl2,
}

impl FileType {
    fn extension(&self) -> &str {
        match self {
            FileType::Pdf => "pdf",
            FileType::Png => "png",
            FileType::Zplii => "zpl",
            FileType::Epl2 => "epl",
        }
    }
}

fn detect_file_type(data: &[u8]) -> Result<FileType, String> {
    // Check for PDF (starts with %PDF-)
    if data.len() >= 4 && &data[0..4] == b"%PDF" {
        return Ok(FileType::Pdf);
    }

    // Check for PNG (starts with PNG signature: 89 50 4E 47 0D 0A 1A 0A)
    if data.len() >= 8 && &data[0..8] == [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A] {
        return Ok(FileType::Png);
    }

    // Convert to string for text-based format detection
    let text = String::from_utf8_lossy(data);

    // Check for ZPLII (Zebra Programming Language)
    // ZPLII commands typically start with ^XA (start) or contain ^XZ (end)
    if text.contains("^XA") || text.contains("^XZ") || text.starts_with("^XA") {
        return Ok(FileType::Zplii);
    }

    // Check for EPL2 (Eltron Programming Language)
    // EPL2 commands typically start with N (print), O (clear), or other single-letter commands
    let trimmed = text.trim();
    if trimmed.starts_with("N") || trimmed.starts_with("O") || trimmed.starts_with("q") {
        // Additional check: EPL2 often has specific command patterns
        if trimmed.contains("\n") || trimmed.contains("\r") {
            let first_line = trimmed.lines().next().unwrap_or("");
            if first_line.len() > 0 && first_line.chars().next().unwrap().is_ascii_alphabetic() {
                return Ok(FileType::Epl2);
            }
        } else if trimmed.len() > 0 && trimmed.chars().next().unwrap().is_ascii_alphabetic() {
            return Ok(FileType::Epl2);
        }
    }

    // If we can't detect, default to PDF (most common for shipping labels)
    // But we should probably return an error instead
    Err("Unable to detect file type. Supported types: PDF, PNG, ZPLII, EPL2".to_string())
}

fn generate_filename(file_type: &FileType) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Format as YYYYMMDD-hh-mm-ss
    let datetime = chrono::DateTime::<chrono::Utc>::from_timestamp(now as i64, 0)
        .unwrap_or_else(|| chrono::Utc::now());

    let formatted = datetime.format("%Y%m%d-%H-%M-%S");
    format!("{}.{}", formatted, file_type.extension())
}

#[tauri::command]
async fn decode_base64(base64_string: String) -> Result<(Vec<u8>, String, String), String> {
    // Remove data URI prefix if present (e.g., "data:image/png;base64,")
    let trimmed = base64_string.trim();
    let start_idx = if let Some(prefix_end) = trimmed.find("base64,") {
        prefix_end + 7
    } else {
        0
    };
    
    // Remove quotes, whitespace, and other non-base64 characters
    // Base64 only contains: A-Z, a-z, 0-9, +, /, = (padding)
    // Also strip leading/trailing quotes that might wrap JSON strings
    let mut cleaned_base64: String = trimmed[start_idx..]
        .trim_matches(|c: char| c == '"' || c == '\'' || c.is_whitespace())
        .chars()
        .filter(|c| {
            c.is_ascii_alphanumeric() || *c == '+' || *c == '/' || *c == '=' || c.is_whitespace()
        })
        .filter(|c| !c.is_whitespace()) // Remove whitespace after filtering
        .collect();
    
    // Remove any remaining quotes that might be embedded
    cleaned_base64.retain(|c| c != '"' && c != '\'');

    if cleaned_base64.is_empty() {
        return Err("Base64 string is empty after cleaning".to_string());
    }

    // Decode base64 string
    let decoded_data = general_purpose::STANDARD
        .decode(&cleaned_base64)
        .map_err(|e| format!("Failed to decode base64: {}. Please ensure the string is valid base64.", e))?;

    if decoded_data.is_empty() {
        return Err("Decoded data is empty".to_string());
    }

    // Detect file type
    let file_type = detect_file_type(&decoded_data)
        .map_err(|e| format!("{}", e))?;

    // Generate filename
    let filename = generate_filename(&file_type);
    let extension = file_type.extension().to_string();

    Ok((decoded_data, filename, extension))
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![decode_base64])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

