// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod extractor;

use extractor::extract_frames;


#[tauri::command]
async fn process_videos(
    input_dir: String,
    output_dir: String,
    skip_start_sec: u64,
    frame_interval_sec: u64,
    preserve_dir_structure: bool,
    create_video_subdir: bool,
) -> Result<Vec<extractor::ProcessResult>, String> {
    extract_frames(
        &input_dir,
        &output_dir,
        skip_start_sec,
        frame_interval_sec,
        preserve_dir_structure,
        create_video_subdir,
    )
    .await
    .map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            process_videos
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

