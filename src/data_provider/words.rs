use crate::dictionary::app_data_dir;
use crate::lang::DictionaryElement;
use rusqlite::Connection;
use std::collections::HashMap;

pub fn create_db() {
    let path = app_data_dir();
    let db_file = path.join("data.db");
    if !db_file.exists() {
        std::fs::File::create(&db_file).unwrap();
        let connection = Connection::open(&db_file).unwrap();
        create_words_table(&connection);
        create_card_stat_table(&connection);
        create_card_set_table(&connection);
    }
}

pub fn add_word(word: &mut DictionaryElement, connection: &Connection) {
    let index = connection
        .query_row(
            "INSERT INTO words (key, value, tags, more) VALUES (?1, ?2, ?3, ?4) RETURNING id",
            (
                &word.key,
                &word.value,
                &word.tags,
                serde_json::to_string(&word.additional).unwrap(),
            ),
            |row| row.get(0),
        )
        .unwrap_or_else(|e| {
            println!("{}", e);
            0
        });

    word.id = index;
}

pub fn update_word(word: &mut DictionaryElement, connection: &Connection) {
    if word.id == 0 {
        add_word(word, &connection);
    } else {
        connection
            .execute(
                "UPDATE words SET key = ?1, value = ?2, tags = ?3, more = ?4 WHERE id = ?5",
                (
                    &word.key,
                    &word.value,
                    &word.tags,
                    serde_json::to_string(&word.additional).unwrap(),
                    &word.id,
                ),
            )
            .unwrap_or_else(|e| {
                println!("{}", e);
                0
            });
    }
}

pub fn delete_word(word: &DictionaryElement, connection: &Connection) {
    if word.id == 0 {
        return;
    }
    connection
        .execute("DELETE FROM words WHERE id = ?1", (&word.id,))
        .unwrap_or_else(|e| {
            println!("{}", e);
            0
        });
}

pub fn load_words(connection: &Connection) -> Vec<DictionaryElement> {
    let mut stmt = connection
        .prepare("SELECT id, key, value, tags, more FROM words")
        .unwrap();
    let word_iter = stmt
        .query_map([], |row| {
            let addinionals: String = row.get(4)?;
            Ok(DictionaryElement {
                id: row.get(0)?,
                key: row.get(1)?,
                value: row.get(2)?,
                tags: row.get(3)?,
                additional: serde_json::from_str::<HashMap<String, String>>(&addinionals).unwrap(),
            })
        })
        .unwrap();

    let mut buffer = vec![];
    for word in word_iter {
        buffer.push(word.unwrap());
    }

    buffer
}

fn create_words_table(conn: &Connection) {
    conn.execute(
        "CREATE TABLE words (
            id   INTEGER PRIMARY KEY AUTOINCREMENT,
            key TEXT NOT NULL,
            value TEXT NOT NULL,
            tags TEXT NOT NULL,
            more TEXT
        )",
        (),
    )
    .unwrap_or_else(|e| {
        println!("{}", e);
        0
    });
}

fn create_card_stat_table(conn: &Connection) {
    conn.execute(
        "CREATE TABLE card_stats (
            id   INTEGER PRIMARY KEY AUTOINCREMENT,
            word_id INTEGER NOT NULL,
            set_id TEXT NOT NULL,
            score INTEGER NOT NULL DEFAULT 1,
            last_opened INTEGER NOT NULL,
            FOREIGN KEY (word_id)  REFERENCES words (id) ON DELETE CASCADE,
            FOREIGN KEY (set_id)  REFERENCES sets (id) ON DELETE CASCADE
        )",
        (),
    )
    .unwrap_or_else(|e| {
        println!("{}", e);
        0
    });
}

fn create_card_set_table(conn: &Connection) {
    conn.execute(
        "CREATE TABLE card_set (
            id   INTEGER PRIMARY KEY AUTOINCREMENT,
            name   TEXT NOT NULL,
            forward TEXT NOT NULL,
            backward TEXT NOT NULL,
            filter TEXT NOT NULL
        )",
        (),
    )
    .unwrap_or_else(|e| {
        println!("{}", e);
        0
    });
}

pub struct Qwerty {}
