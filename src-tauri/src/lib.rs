mod commands;
mod db_logic;
mod models;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::load_parquet_schema,
            commands::get_page_data,
            commands::get_file_metadata,
            commands::run_sql,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
