use std::collections::HashMap;
use chrono::{DateTime, Utc};
use iced::time::now;
use rusqlite::{Connection, ToSql};
use rusqlite::types::ToSqlOutput;
use crate::lang::{CardSet, CardStatistics, DictionaryElement};
use crate::repetitions::CardSetSettings;

pub fn load_stats_of_set(set: &CardSetSettings, connection: &Connection) -> Vec<CardStatistics> {
    let mut stmt = connection.prepare("SELECT id, word_id, score, last_opened FROM card_stats WHERE set_id = ?1").unwrap();
    let iter = stmt.query_map((set.id,), |row| {
        Ok(CardStatistics {
            id: row.get(0)?,
            word_id: row.get(1)?,
            score: row.get(2)?,
            last_open: row.get(2)?,
        })
    }).unwrap();

    let mut buffer = vec![];
    for word in iter {
        buffer.push(word.unwrap());
    }

    buffer
}

pub fn add_set(set: &mut CardSetSettings, connection: &Connection) {
    let index = connection
        .query_row(
            "INSERT INTO card_set (name, forward, backward, filter) VALUES (?1, ?2, ?3, ?4) RETURNING id",
            (
                &set.name,
                &set.forward,
                &set.backward,
                &set.filter,
            ),
            |row| row.get(0)
        )
        .unwrap_or_else(|e| {println!("{}", e); 0});

    set.id = index;
}

pub fn update_card_set(set: &mut CardSetSettings, connection: &Connection){
    if set.id == 0 {
        add_set(set, &connection);
    }

    else {
        connection
            .execute(
                "UPDATE card_set SET name = ?1, forward = ?2, backward = ?3, filter = ?4 WHERE id = ?5",
                (
                    &set.name,
                    &set.forward,
                    &set.backward,
                    &set.filter,
                    &set.id
                ),
            )
            .unwrap_or_else(|e| {println!("{}", e); 0});
    }
}

pub fn delete_set(set: &CardSetSettings, connection: &Connection) {
    if set.id == 0 {
        return;
    }
    connection
        .execute("DELETE FROM card_set WHERE id = ?1", (&set.id,))
        .unwrap_or_else(|e| {
            println!("{}", e);
            0
        });
}
