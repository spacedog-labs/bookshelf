use rusqlite::{params_from_iter, Connection, Error, Row, ToSql};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::models::book::Book;

/// Current version of the DB Schema.
/// This will change between major versions and tables will need to uptake.
pub const SCHEMA_VERSION: SchemaVersion = SchemaVersion::One;

/// The `SchemaVersion` Enum defines what schema version the source code is on.
/// This is called Migrations in some program/frameworks.
/// This enum is only additive and if changed, all `Table<T>` must uptake it.
pub enum SchemaVersion {
    One = 1,
}

impl SchemaVersion {
    /// Converts from `i32` to `SchemaVersion`
    pub fn from_i32(val: i32) -> SchemaVersion {
        match val {
            1 => SchemaVersion::One,
            _ => todo!(),
        }
    }

    /// Converts from `SchemaVersion` to `i32`
    pub fn to_i32(version: SchemaVersion) -> i32 {
        match version {
            SchemaVersion::One => 1,
        }
    }
}

/// Struct used to handle locking/releasing the db connection.
pub struct Database(pub Arc<Mutex<Connection>>);

impl Database {
    /// Used to query data from sqlite.
    /// `T` must implement `FromRow<T>` to deserialize correctly.
    pub fn query<T: FromRow<T>>(&self, stmt: &str) -> Result<Vec<T>, Error> {
        let con = &self.0.lock().unwrap();

        let mut stmt = con.prepare(stmt)?;
        let rows = stmt.query_map([], T::from_row)?;

        let mut ret_rows = Vec::<T>::new();

        for row_res in rows {
            if let Ok(row) = row_res {
                ret_rows.push(row);
            }
        }

        return Ok(ret_rows);
    }

    /// Used to insert records to the sqlite db.
    /// `T` must implement `ToRow<T>` to correctly serialize.
    pub fn insert<T: ToRow<T>>(&self, item: &T, stmt: &str) -> Result<(), Error> {
        let con = &self.0.lock().unwrap();
        con.execute(stmt, params_from_iter(<T>::to_row(item)))?;
        Ok(())
    }

    /// Used to get user_version pragma to track data migrations.
    pub fn get_pragma(&self) -> Result<i32, Error> {
        let con = &self.0.lock().unwrap();
        let user_version: i32 = con.pragma_query_value(None, "user_version", |row| row.get(0))?;
        Ok(user_version)
    }

    /// Used to set user_verions to track data migration.
    pub fn set_pragma(&self, version: i32) -> Result<(), Error> {
        let con = &self.0.lock().unwrap();
        con.pragma(None, "user_version", version, |_| Ok(()))?;
        Ok(())
    }

    /// Used to execute DB code without returning any data.
    /// Schema changes use this.
    pub fn execute(&self, stmt: &str) -> Result<(), rusqlite::Error> {
        let con = &self.0.lock().unwrap();

        con.execute(stmt, [])?;

        return Ok(());
    }
}

/// Defines how to convert from `Row` to `T`
pub trait FromRow<T> {
    fn from_row(row: &Row<'_>) -> Result<T, Error>;
}

/// Defines ghow to convert from `T` to `Row`
pub trait ToRow<T> {
    fn to_row(item: &T) -> Vec<&dyn ToSql>;
}

/// `Table<T>` is defined for all structs that wish to be tables in sqlite.
pub trait Table<T> {
    /// Implementations of this trait allow incremental schema upgrades for the db
    /// and should match on `SchemaVersion` and cover changes between each version
    /// or no-op.
    fn update(database: &Database, s_version: SchemaVersion) -> Result<(), Error>;
}

/// Creates sqllite DB and safe connection for Management.
pub fn create_db(db_file: PathBuf) -> Database {
    return Database(Arc::new(Mutex::new(Connection::open(db_file).unwrap())));
}

/// Fetches Pragma and runs through each table to run migration code.
pub fn perform_data_migration(db: &Database) -> Result<(), Error> {
    let mut user_version = db.get_pragma()?;

    while user_version < SchemaVersion::to_i32(SCHEMA_VERSION) {
        user_version += 1;
        Book::update(db, SchemaVersion::from_i32(user_version))?;
        db.set_pragma(user_version)?;
    }

    Ok(())
}
