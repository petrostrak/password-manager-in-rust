use crate::Password;
use rusqlite::{params, Connection, DatabaseName};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(key: String) -> Result<Database, rusqlite::Error> {
        let path = dirs::config_dir().unwrap().join("my_passmng");
        let conn = Connection::open(path)?;
        // set password to our database. without this passphrase database is not readable
        conn.pragma_update(Some(DatabaseName::Main), "KEY", key)?;
        let db = Database { conn };
        db.create_table()?;
        Ok(db)
    }

    pub fn create_table(&self) -> Result<(), rusqlite::Error> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS passwords(
                    id INTEGER PRIMARY KEY,
                    title TEXT NOT NULL,
                    username TEXT NOT NULL,
                    password TEXT NOT NULL
                )
            ",
        )?;
        Ok(())
    }
}