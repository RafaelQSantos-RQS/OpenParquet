mod commands;
mod db_logic;
mod error;
mod models;
mod state;
mod validation;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::new().expect("Failed to initialize database connection");

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            commands::load_parquet_schema,
            commands::get_page_data,
            commands::get_file_metadata,
            commands::run_sql,
            commands::export_data,
            commands::get_multi_file_metadata,
            commands::get_file_list_metadata,
            commands::get_multi_file_page_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
