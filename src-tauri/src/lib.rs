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
            commands::open_dataset,
            commands::get_page,
            commands::run_sql,
            commands::export_dataset,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
