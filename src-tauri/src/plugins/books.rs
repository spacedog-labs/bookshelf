use crate::db::Database;
use epub::doc::EpubDoc;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

use crate::models::book::Book;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("books")
        .invoke_handler(tauri::generate_handler![get_books, add_book, test_book])
        .build()
}

#[tauri::command]
fn get_books(db: tauri::State<Database>) -> Result<Vec<Book>, String> {
    match db.query::<Book>("SELECT id, title FROM book") {
        Ok(books) => {
            return Ok(books);
        }
        Err(error) => return Err(error.to_string()),
    }
}

#[tauri::command]
fn add_book(book: Book, db: tauri::State<Database>) -> Result<(), String> {
    match db.insert::<Book>(&book, "INSERT INTO book (id, title) VALUES (?1, ?2)") {
        Ok(()) => Ok(()),
        Err(error) => return Err(error.to_string()),
    }
}

#[tauri::command]
fn test_book(location: String) -> Result<String, String> {
    let doc = EpubDoc::new(location);
    let mut doc = doc.unwrap();

    doc.go_next().unwrap();

    let title = doc.get_current_str();

    match title {
        Ok(title) => {
            return Ok(title);
        }
        Err(_) => {
            return Err("Could not load book.".to_string());
        }
    }
}
