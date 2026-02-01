use rusqlite::{params, Connection, Result};
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
                id      INTEGER PRIMARY KEY AUTOINCREMENT,
                game_id INTEGER NOT NULL,
                type    INTEGER NOT NULL,
                answer  TEXT NOT NULL,
                guesses TEXT,
                FOREIGN KEY(game_id) REFERENCES games(id)
            )",
            [],
        )?;

        Ok(())
    }

    pub fn add_game(&self, game_type: u8, answer: &str,) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO games DEFAULT VALUES", []
        )?;

        let id = self.conn.last_insert_rowid();
        self.conn.execute(
            "INSERT INTO status (
                game_id,
                type,
                answer
            ) VALUES (?1, ?2, ?3)",
            params![id, game_type, answer],
        )?;
        
        Ok(id)
    }
}