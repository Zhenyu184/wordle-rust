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
        let encrypted_answer = crate::misc::encrypt(answer);
        self.conn.execute(
            "INSERT INTO status (game_id, type, answer) VALUES (?1, ?2, ?3)",
            params![game_id, game_type, encrypted_answer],
        )?;
        Ok(())
    }

    fn select_games(&self) -> Result<Vec<(i64, i64, u8, bool)>> {
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

    fn select_status(&self, id: i64) -> Result<(i64, i64, u8, bool, String, Option<String>)> {
        let mut stmt = self.conn.prepare(
            "SELECT g.id, g.time, s.type, s.is_over, s.answer, s.guesses
             FROM   games g 
             JOIN status s ON g.id = s.game_id
             WHERE g.id = ?1"
        )?;
        
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            let answer: String = row.get(4)?;
            let decrypted_answer = crate::misc::decrypt(&answer);
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                decrypted_answer,
                row.get(5)?,
            ))
        } else {
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }

    fn update_guesses_in_status(&self, game_id: i64, guesses: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE status SET guesses = ?1 WHERE game_id = ?2",
            params![guesses, game_id],
        )?;
        Ok(())
    }

    fn update_over_in_status(&self, game_id: i64, is_over: bool) -> Result<()> {
        self.conn.execute(
            "UPDATE status SET is_over = ?1 WHERE game_id = ?2",
            params![is_over, game_id],
        )?;
        Ok(())
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
        let ret = self.select_games()?;
        Ok(ret)
    }

    pub fn get_status(&self, id: i64) -> Result<(i64, i64, u8, bool, String, Option<String>)> {
        let game = self.select_status(id)?;
        Ok(game)
    }

    pub fn append_guesses(&self, game_id: i64, guess: &str) -> Result<()> {
        let (_, _, _, _, _, guesses) = self.select_status(game_id)?;
        let mut guesses = guesses.unwrap_or_default();
        if !guesses.is_empty() {
            guesses.push(',');
        }
        
        guesses.push_str(guess);
        self.update_guesses_in_status(game_id, &guesses)
    }

    pub fn set_game_over(&self, game_id: i64) -> Result<()> {
        self.update_over_in_status(game_id, true)
    }

    pub fn delete_all(&self) -> Result<()> {
        self.delete_games()?;
        self.delete_status()?;
        self.delete_sqlite_sequence()?;
        Ok(())
    }
}