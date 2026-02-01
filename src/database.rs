use std::sync::{Mutex, MutexGuard, OnceLock};
use rusqlite::{params, Connection, Result};

pub struct Database {
    conn: Connection,
}

static DB: OnceLock<Mutex<Database>> = OnceLock::new();

impl Database {

    // private

    fn new() -> Result<Self> {
        let conn = Connection::open("wordle.db")?;
        Ok(Database { conn })
    }
    
    fn instance() -> &'static Mutex<Database> {
        DB.get_or_init(|| Mutex::new(Database::new().expect("Failed to connect to database")))
    }

    fn create_games(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS games (
                id   INTEGER PRIMARY KEY AUTOINCREMENT,
                time INTEGER DEFAULT (unixepoch())
            )",
            [],
        )?;
        Ok(())
    }

    fn delete_games(&self) -> Result<()> {
        self.conn.execute("DELETE FROM games", [])?;
        Ok(())
    }

    fn create_status(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS status (
                id      INTEGER PRIMARY KEY AUTOINCREMENT,
                game_id INTEGER NOT NULL,
                type    INTEGER NOT NULL,
                answer  TEXT    NOT NULL,
                guesses TEXT,
                is_over BOOLEAN NOT NULL DEFAULT 0,
                FOREIGN KEY(game_id) REFERENCES games(id)
            )",
            [],
        )?;
        Ok(())
    }

    fn delete_status(&self) -> Result<()> {
        self.conn.execute("DELETE FROM status", [])?;
        Ok(())
    }

    fn delete_sqlite_sequence(&self) -> Result<()> {
        self.conn.execute("DELETE FROM sqlite_sequence", [])?;
        Ok(())
    }

    fn insert_games(&self) -> Result<i64> {
        self.conn.execute("INSERT INTO games DEFAULT VALUES", [])?;
        Ok(self.conn.last_insert_rowid())
    }

    fn insert_status(&self, game_id: i64, game_type: u8, answer: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO status (game_id, type, answer) VALUES (?1, ?2, ?3)",
            params![game_id, game_type, answer],
        )?;
        Ok(())
    }

    fn select_games_status(&self) -> Result<Vec<(i64, i64, u8, bool)>> {
        let mut stmt = self.conn.prepare(
            "SELECT g.id, g.time, s.type, s.is_over 
             FROM   games g 
             JOIN status s ON g.id = s.game_id"
        )?;
        
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
            ))
        })?;

        let mut ret = Vec::new();
        for row in rows {
            ret.push(row?);
        }

        Ok(ret)
    }

    // public

    pub fn connect() -> MutexGuard<'static, Database> {
        Self::instance().lock().unwrap()
    }

    pub fn init(&self) -> Result<()> {
        self.create_games()?;
        self.create_status()?;
        Ok(())
    }

    pub fn add_game(&self, game_type: u8, answer: &str,) -> Result<i64> {
        let id = self.insert_games()?;
        self.insert_status(id, game_type, answer)?;
        Ok(id)
    }

    pub fn get_games(&self) -> Result<Vec<(i64, i64, u8, bool)>> {
        let ret = self.select_games_status()?;
        Ok(ret)
    }

    pub fn delete_all(&self) -> Result<()> {
        self.delete_games()?;
        self.delete_status()?;
        self.delete_sqlite_sequence()?;
        Ok(())
    }
}