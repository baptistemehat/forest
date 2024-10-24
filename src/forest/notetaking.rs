use super::dbutils;
use chrono::{DateTime, Local};
use edit as default_editor;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, BufRead};

use super::types;

/// Create a new note linked to the current tree
///
/// # Errors
/// Returns an error if the forest is empty or if the given tree name does not exist in forest
///
/// # Panics
/// This function may panic if database operations fail
pub async fn add(tree_name: Option<String>) -> Result<(), Box<dyn Error>> {
    let new_note_uid = types::generate_uid();

    // create a new note on file system
    let new_note_path = match dbutils::get_note_path(&new_note_uid) {
        Some(path) => path,
        None => panic!("Could not create a new note on file system."),
    };

    // open default editor for user to edit the new note
    if let Err(e) = default_editor::edit_file(new_note_path) {
        panic!("Failed to open new note in default editor: {e}");
    }

    // add this note to db
    let pool = dbutils::load_db().await;

    let tree_name = match tree_name {
        Some(name) => name,
        None => match dbutils::get_current_tree_name(&pool).await{
            Some(current_tree_name) => current_tree_name,
            None => return Err(
                "no current tree found. it seems like your forest is empty.\nconsider adding a tree."
                .into(),
            ),
        },
    };

    let mut conn = pool
        .acquire()
        .await
        .expect("Acquiring connection to database should succeed");

    // get root task id of current tree
    let query_result = sqlx::query!(
        r#"
        SELECT "id"
        FROM task
        WHERE tree_name = ? AND "left" = 1;
        "#,
        tree_name,
    )
    .fetch_one(&mut *conn)
    .await;

    let task = match query_result {
        Ok(record) => record,
        Err(query_error) => match query_error {
            sqlx::Error::RowNotFound => {
                return Err(format!("Tree '{tree_name}' not found").into());
            }
            other_error => panic!("Database query failed: {other_error}"),
        },
    };

    // insert new note into database
    let now = Local::now();
    let date = now.timestamp_millis();
    let task_id = task.id;
    let query_result = sqlx::query!(
        r#"
        INSERT INTO note("id", "date", "task_id")
        VALUES (?, ?, ?);
        "#,
        new_note_uid,
        date,
        task_id,
    )
    .execute(&mut *conn)
    .await;

    // error handling
    match query_result {
        Ok(result) => {
            if result.rows_affected() != 1 {
                panic!("Creating a new note should affect at least one row (the isnerted note)");
            }
        }
        Err(query_error) => match query_error {
            sqlx::Error::Database(db_error) => match db_error.kind() {
                sqlx::error::ErrorKind::UniqueViolation => {
                    panic!("Note id should be unique");
                }
                _ => panic!("Database query failed: {db_error}"),
            },
            other_error => panic!("Database query failed: {other_error}"),
        },
    }

    println!("Added a new note to tree '{tree_name}'");

    Ok(())
}

/// List all notes
///
/// # Errors
/// Returns an error if no notes exist in the forest
///
/// # Panics
/// This function may panic if database operations fail
pub async fn list(show_uid: bool) -> Result<(), Box<dyn Error>> {
    let pool = dbutils::load_db().await;

    let mut conn = pool
        .acquire()
        .await
        .expect("Acquiring connection to database should succeed");

    // foreach note, get date, tree name and note id
    let query_result = sqlx::query!(
        r#"
        -- foreach note, get date, tree name and note id

        SELECT tree_name, date, n.id
        FROM note n INNER JOIN task t ON n.task_id = t.id
        ORDER BY date DESC;
        "#
    )
    .fetch_all(&mut *conn)
    .await;

    // error handling
    let records = match query_result {
        Ok(records) => {
            if records.is_empty() {
                return Err("You have no notes to display".into());
            } else {
                records
            }
        }
        Err(query_error) => panic!("Database query failed: {query_error}"),
    };

    // get length of the longest tree name associated with a note for pretty alignment
    let max_tree_name_length = records
        .iter()
        .max_by(|a, b| a.tree_name.len().cmp(&b.tree_name.len()))
        .map(|a| a.tree_name.len())
        .expect(
            "At this point in the function, the itterator should point to a non empty collection",
        );

    // print each note
    for note in records {
        if show_uid {
            print!("{}   ", note.id);
        }

        // print note date
        let note_date: DateTime<Local> = DateTime::from_timestamp_millis(note.date).unwrap().into();
        print!("{}   ", note_date.format("%Y-%m-%d %H:%M:%S"));

        // print tree_name with padding for alignment
        print!(
            "{:width$}   ",
            note.tree_name,
            width = &max_tree_name_length
        );

        // try to open note file to display its first line
        let note_file_path = dbutils::get_note_path(&note.id)
            .expect("A note file should be associated with each note in database");
        let note_file = File::open(note_file_path).expect("Path to the note file should exist");
        let reader = io::BufReader::new(note_file);

        // try to get first line of file if any to print a "note preview"
        match reader.lines().next() {
            Some(line) => println!("{}", line.expect("Failed to read first line of note file")),
            None => println!("\x1b[1mEmpty Note\x1b[0m"),
        }
    }

    Ok(())
}

/// Remove a note
///
/// # Errors
/// Returns an error if the note does not exist
///
/// # Panics
/// This function may panic if database operations fail
pub async fn remove(uid: &types::Uid) -> Result<(), Box<dyn Error>> {
    let pool = dbutils::load_db().await;

    let mut conn = pool
        .acquire()
        .await
        .expect("Acquiring connection to database should succeed");

    // remove note from file system
    let note_file_path = dbutils::get_note_path(uid)
        .expect("A note file should be associated with each note in database");
    fs::remove_file(note_file_path).expect("Failed to remove note file from filesystem");

    // remove note from the note table
    let query_result = sqlx::query!(
        r#"
        DELETE FROM note
        WHERE id = ?;
        "#,
        uid
    )
    .execute(&mut *conn)
    .await;

    match query_result {
        Ok(result) => {
            if result.rows_affected() < 1 {
                return Err(format!("Note '{uid}' not found").into());
            }
        }
        Err(query_error) => panic!("Database query failed: {query_error}"),
    }

    println!("Removed note {}", uid);

    Ok(())
}

/// Edit a note
///
/// # Errors
/// Returns an error if the note does not exist
pub async fn edit(uid: &types::Uid) -> Result<(), Box<dyn Error>> {
    let note_path = match dbutils::get_note_path(uid) {
        Some(path) => path,
        None => return Err(format!("Could not find note {uid}").into()),
    };

    // open default editor for user to edit the note
    if let Err(e) = default_editor::edit_file(note_path) {
        panic!("Failed to open new note in default editor: {e}");
    }

    Ok(())
}

/// Show the content of the note
///
/// # Errors
/// Returns an error if the note does not exist
pub async fn show(uid: &types::Uid) -> Result<(), Box<dyn Error>> {
    let pool = dbutils::load_db().await;

    let mut conn = pool
        .acquire()
        .await
        .expect("Acquiring connection to database should succeed");

    // get tree name and date of the note
    let query_result = sqlx::query!(
        r#"
        -- get tree name and date of the note

        SELECT tree_name, date
        FROM note n INNER JOIN task t ON n.task_id = t.id
        WHERE n.id = ?
        ORDER BY date DESC;
        "#,
        uid
    )
    .fetch_one(&mut *conn)
    .await;

    // error handling
    let note = match query_result {
        Ok(r) => r,
        Err(query_error) => match &query_error {
            sqlx::Error::RowNotFound => return Err(format!("Note '{uid}' not found").into()),
            _ => panic!("Database query failed: {query_error}"),
        },
    };

    // get note path
    let note_path = match dbutils::get_note_path(uid) {
        Some(path) => path,
        None => return Err(format!("Could not find note {uid}").into()),
    };

    let note_content = fs::read_to_string(&note_path)
        .expect("At this point in the function, the note file should exist");

    let note_date: DateTime<Local> = DateTime::from_timestamp_millis(note.date).unwrap().into();
    println!(
        "\x1b[1mNote:\x1b[0m   {}   {}   ",
        note.tree_name,
        note_date.format("%Y-%m-%d %H:%M:%S")
    );
    println!("{note_content}");

    Ok(())
}
