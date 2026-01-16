#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod project;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::create_project,
            commands::open_project,
            commands::list_projects
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
