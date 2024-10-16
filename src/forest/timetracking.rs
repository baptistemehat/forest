use chrono::{DateTime, Local};
use std::error::Error;

use super::dbutils;
use super::types;

/// Starts recording time spent on a tree
///
/// # Errors
/// Returns an error if the forest is empty or if the given tree name does not exist in forest
///
/// # Panics
/// This function may panic if database operations fail
pub async fn start(tree_name: Option<String>) -> Result<(), Box<dyn Error>> {
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

    // get root task id of tree
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

    // stop any previous recording
    let _ = stop().await;

    // get current local time
    let now = Local::now();

    // insert a new time frame into the frame table
    let new_frame_id = types::generate_uid();
    let start_time = now.timestamp_millis();
    let end_time: Option<i32> = None;
    let root_task_id = task.id;
    let query_result = sqlx::query!(
        r#"
        INSERT INTO frame("id", "start", "end", "task_id")
        VALUES(?, ?, ?, ?);
        "#,
        new_frame_id,
        start_time,
        end_time,
        root_task_id,
    )
    .execute(&mut *conn)
    .await;

    // error handling
    match query_result {
        Ok(result) => {
            if result.rows_affected() != 1 {
                panic!("A single line should have been inserted into the frame table");
            }
        }
        Err(query_error) => match query_error {
            sqlx::Error::Database(db_error) => match db_error.kind() {
                sqlx::error::ErrorKind::UniqueViolation => {
                    panic!("Frame id should be unique");
                }
                _ => panic!("Database query failed: {db_error}"),
            },
            other_error => panic!("Database query failed: {other_error}"),
        },
    }

    println!(
        "Started recording time on tree '{tree_name}' at {}",
        now.format("%Y-%m-%d %H:%M:%S")
    );

    Ok(())
}

/// Stops the current time recording(s)
///
/// # Errors
/// Returns an error if no recording were started
///
/// # Panics
/// This function may panic if database operations fail
pub async fn stop() -> Result<(), Box<dyn Error>> {
    let pool = dbutils::load_db().await;

    let mut conn = pool
        .acquire()
        .await
        .expect("Acquiring connection to database should succeed");

    // get all started time tracking frames
    // note: there should only be one simultaneous time recording  at any time
    let query_result = sqlx::query!(
        r#"
        SELECT "start", "tree_name"
        FROM frame f INNER JOIN task t ON f.task_id = t.id
        WHERE f."end" is NULL;
        "#,
    )
    .fetch_all(&mut *conn)
    .await;

    // error handling
    let started_frames = match query_result {
        Ok(records) => {
            if records.is_empty() {
                return Err("No recording was started".into());
            } else {
                records
            }
        }
        Err(query_error) => panic!("Database query failed: {query_error}"),
    };

    // update end time of started time tracking frame(s)
    let end_time = Local::now().timestamp_millis();
    let query_result = sqlx::query!(
        r#"
            UPDATE frame
            SET "end" = ?
            FROM frame f INNER JOIN task t ON f.task_id = t.id
            WHERE f."end" is NULL;
            "#,
        end_time
    )
    .execute(&mut *conn)
    .await;

    // error handling
    match query_result {
        Ok(result) => {
            if result.rows_affected() < 1 {
                panic!("Stopping a recording should at least update one row");
            }
        }
        Err(query_error) => panic!("Database query failed: {query_error}"),
    };

    // in case multiple time recordings were started
    // print stopping message for each
    for frame in started_frames {
        let start_time: DateTime<Local> =
            DateTime::from_timestamp_millis(frame.start).unwrap().into();
        println!(
            "Stopped recording time on tree '{}' (Sarted at {})",
            frame.tree_name,
            start_time.format("%Y-%m-%d %H:%M:%S")
        );
    }

    Ok(())
}

/// Prints the name of the current tree and current time tracking frames
///
/// # Errors
/// Returns an error if the forest is empty
///
/// # Panics
/// This function may panic if database operations fail
pub async fn status() -> Result<(), Box<dyn Error>> {
    let pool = dbutils::load_db().await;

    let current_tree_name = match dbutils::get_current_tree_name(&pool).await {
        Some(name) => name,
        None => return Err(
            "No current tree found. It seems like your forest is empty.\nConsider adding a tree."
                .into(),
        ),
    };

    let mut conn = pool
        .acquire()
        .await
        .expect("Acquiring connection to database should succeed");

    // get current frame if any
    let query_result = sqlx::query!(
        r#"
        SELECT "start", "tree_name"
        FROM frame f
        INNER JOIN task t ON f.task_id = t.id
        WHERE f."end" is NULL;
        "#,
    )
    .fetch_optional(&mut *conn)
    .await;

    // error handling
    let current_frame = match query_result {
        Ok(record) => record,
        Err(query_error) => panic!("Database query failed: {query_error}"),
    };

    // print current tree
    println!("On tree '{current_tree_name}'");

    // print current time tracking recording if any
    match current_frame {
        Some(frame) => {
            let start_time: DateTime<Local> =
                DateTime::from_timestamp_millis(frame.start).unwrap().into();
            println!(
                "Recording time on tree '{}'. Sarted at {}",
                frame.tree_name,
                start_time.format("%Y-%m-%d %H:%M:%S")
            );
        }
        None => println!("No recording started."),
    }

    Ok(())
}
