use crate::lang::CardStatistics;
use crate::repetitions::CardSetSettings;
use rusqlite::Connection;

pub fn load_stats_of_set(set: &CardSetSettings, connection: &Connection) -> Vec<CardStatistics> {
    let mut stmt = connection.prepare("SELECT id, word_id, score, last_opened FROM card_stats WHERE set_id = ?1").unwrap();
    let iter = stmt.query_map((set.id,), |row| {
        Ok(CardStatistics {
            id: row.get(0)?,
            word_id: row.get(1)?,
            set_id: set.id,
            score: row.get(2)?,
            last_open: row.get(3)?,
        })
    }).unwrap();

    let mut buffer = vec![];
    for word in iter {
        buffer.push(word.unwrap());
    }

    buffer
}

pub fn add_stat(stat: &mut CardStatistics, connection: &Connection) {
    let index = connection
        .query_row(
            "INSERT INTO card_stats (word_id, set_id, score, last_opened) VALUES (?1, ?2, ?3, ?4) RETURNING id",
            (
                &stat.word_id,
                &stat.set_id,
                &stat.score,
                &stat.last_open,
            ),
            |row| row.get(0)
        )
        .unwrap_or_else(|e| {println!("{}", e); 0});

    stat.id = index;
}

pub fn update_stat(stat: &mut CardStatistics, connection: &Connection){
    if stat.id == 0 {
        add_stat(stat, &connection);
    }

    else {
        connection
            .execute(
                "UPDATE card_stats SET word_id = ?1, set_id = ?2, score = ?3, last_opened = ?4 WHERE id = ?5",
                (
                    &stat.word_id,
                    &stat.set_id,
                    &stat.score,
                    &stat.last_open,
                    &stat.id
                ),
            )
            .unwrap_or_else(|e| {println!("{}", e); 0});
    }
}


pub fn update_stat_score(stat: &CardStatistics, connection: &Connection){
        connection
            .execute(
                "UPDATE card_stats SET score = ?1, last_opened = ?2 WHERE id = ?3",
                (
                    &stat.score,
                    &stat.last_open,
                    &stat.id
                ),
            )
            .unwrap_or_else(|e| {println!("{}", e); 0});
}

pub fn delete_set(set: &CardSetSettings, connection: &Connection) {
    if set.id == 0 {
        return;
    }
    connection
        .execute("DELETE FROM card_stats WHERE id = ?1", (&set.id,))
        .unwrap_or_else(|e| {
            println!("{}", e);
            0
        });
}
