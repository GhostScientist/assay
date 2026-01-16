#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod eval;
mod project;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::create_project,
            commands::open_project,
            commands::list_projects,
            commands::list_evals
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
