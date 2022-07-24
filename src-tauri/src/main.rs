#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use rusqlite::Error;
use tauri::api::path;

mod db;
mod models;
mod plugins;

fn main() -> Result<(), Error> {
    let context = tauri::generate_context!();

    let app_dir = path::app_dir(context.config()).unwrap();
    let mut db_path = app_dir.clone();
    db_path.push("bookshelf.db");

    let db = db::create_db(db_path);
    db::perform_data_migration(&db)?;

    tauri::Builder::default()
        .manage(db)
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(plugins::books::init())
        .run(context)
        .expect("error while running tauri application");

    Ok(())
}
