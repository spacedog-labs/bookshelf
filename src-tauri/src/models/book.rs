use crate::db::{Database, FromRow, SchemaVersion, Table, ToRow};
use serde::{Deserialize, Serialize};

use rusqlite::{
    params,
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef},
    Error, Row, ToSql,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    id: i32,
    title: String,
    book_type: BookType,
}

impl FromRow<Book> for Book {
    fn from_row(row: &Row) -> Result<Book, Error> {
        Ok(Book {
            id: row.get(0)?,
            title: row.get(1)?,
            book_type: row.get(2)?,
        })
    }
}

impl ToRow<Book> for Book {
    fn to_row(book: &Book) -> Vec<&dyn ToSql> {
        params! {
            book.id,
            book.title
        }
        .to_vec()
    }
}

impl Table<Book> for Book {
    fn update(db: &Database, s_version: SchemaVersion) -> Result<(), Error> {
        match s_version {
            SchemaVersion::One => {
                db.execute(
                    "CREATE TABLE IF NOT EXISTS book (id INTEGER, title TEXT, booktype INTEGER)",
                )?;
            }
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum BookType {
    EPub = 1,
}

impl FromSql for BookType {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        match i32::column_result(value) {
            Ok(num) => match num {
                1 => Ok(BookType::EPub),
                _ => Err(FromSqlError::InvalidType),
            },
            Err(err) => Err(err),
        }
    }
}

impl ToSql for BookType {
    fn to_sql(&self) -> Result<ToSqlOutput, Error> {
        Ok((*self as u8).into())
    }
}
