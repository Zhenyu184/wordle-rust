use rusqlite::{Connection, Result};
use std::sync::{Mutex, MutexGuard, OnceLock};

pub struct Database {
    conn: Connection,
}

static DB: OnceLock<Mutex<Database>> = OnceLock::new();

impl Database {
    fn new() -> Result<Self> {
        let conn = Connection::open("wordle.db")?;
        Ok(Database { conn })
    }
    
    fn instance() -> &'static Mutex<Database> {
        DB.get_or_init(|| Mutex::new(Database::new().expect("Failed to connect to database")))
    }

    pub fn connect() -> MutexGuard<'static, Database> {
        Self::instance().lock().unwrap()
    }

    pub fn init(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS games (
                id   INTEGER PRIMARY KEY AUTOINCREMENT,
                time INTEGER DEFAULT (unixepoch())
            )",
            [],
        )?;
        
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS status (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                game_id INTEGER NOT NULL,
                answer TEXT NOT NULL,
                guesses TEXT,
                FOREIGN KEY(game_id) REFERENCES games(id)
            )",
            [],
        )?;
        Ok(())
    }
}