use chrono::{
    DateTime, Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta, Utc, Weekday,
};
use std::error::Error;

use super::ansi;
use super::dbutils;
use super::notetaking;
use super::tree;
use forest_types::Uid;

const DATE_FORMAT: &str = "%Y-%m-%d";
const TIME_FORMAT: &str = "%H:%M";

/// Returns a human-friendly representation of WHEN was the given datetime compared to now
fn when(datetime: DateTime<Local>) -> String {
    let delta = Local::now() - datetime;

    if delta.num_weeks() > 1 {
        return format!("{} weeks ago", delta.num_weeks());
    } else if delta.num_days() > 6 {
        return String::from("a week ago");
    } else if delta.num_days() > 1 {
        return format!("{} days ago", delta.num_days());
    } else if delta.num_hours() > 23 {
        return String::from("a day ago");
    } else if delta.num_hours() > 1 {
        return format!("{} hours ago", delta.num_hours());
    } else if delta.num_minutes() > 59 {
        return String::from("an hour ago");
    } else if delta.num_minutes() > 1 {
        return format!("{} minutes ago", delta.num_minutes());
    }
    String::from("just now")
}

/// Parses a user datetime and returns the parse Datetime.
/// If input user datetime is `None`, returns `Local::now()`.
///
/// Input format are "%Y-%m-%d %H:%M" "%H:%M".
///
/// # Errors
/// Returns an error if the input string is ill-formed
fn parse_user_datetime_or(
    user_datetime_str: &Option<String>,
    or_value: DateTime<Local>,
) -> Result<DateTime<Local>, Box<dyn Error>> {
    // Date time format to use for parsing user input
    let mut datetime_format = String::from(DATE_FORMAT);
    datetime_format.push(' ');
    datetime_format.push_str(TIME_FORMAT);

    let datetime: DateTime<Local> = match user_datetime_str {
        // if user datetime provided, parse it
        Some(datetime) => {
            // try to parse DATE AND TIME format "%Y-%m-%d %H:%M"
            let naive_datetime = match NaiveDateTime::parse_from_str(datetime, &datetime_format) {
                Ok(naive_datetime) => naive_datetime,

                // if failed to parse DATE AND TIME format "%Y-%m-%d %H:%M",
                // try to parse only TIME format "%H:%M"
                Err(_) => match NaiveTime::parse_from_str(datetime, TIME_FORMAT) {
                    Ok(naive_time) => Utc::now().date_naive().and_time(naive_time),

                    // failed to parse both formats
                    Err(_) => {
                        return Err(format!(
                            "Illegal date format. Date format should be \"{}\" or \"{}\"",
                            datetime_format, TIME_FORMAT
                        )
                        .into());
                    }
                },
            };

            // fit the parsed date time to the local timezone
            naive_datetime
                .and_local_timezone(Local)
                .single()
                .expect("Timezone convertion should not fail. See https://docs.rs/chrono/latest/chrono/offset/type.MappedLocalTime.html#variant.None")
        }
        // if no user datetime provided, return current time
        None => or_value,
    };

    Ok(datetime)
}
/// Starts recording time spent on a tree
///
/// # Errors
/// Returns an error if the forest is empty or if the given tree name does not exist in forest
///
/// # Panics
/// This function may panic if database operations fail
pub async fn start(
    tree_name: Option<String>,
    datetime: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let start_datetime = parse_user_datetime_or(&datetime, Local::now())?;

    let pool = dbutils::load_db().await;

    // get tree name if one was provided, current tree name otherwise
    let tree_name = match tree_name {
        Some(name) => name,
        None => dbutils::get_current_tree_name(&pool).await?,
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
    let _ = stop(datetime, true).await;

    // insert a new time frame into the frame table
    let new_frame_uid = Uid::new();
    let start_time = start_datetime.timestamp_millis();
    let end_time: Option<i32> = None;
    let root_task_id = task.id;
    let query_result = sqlx::query!(
        r#"
        INSERT INTO frame("id", "start", "end", "task_id")
        VALUES(?, ?, ?, ?);
        "#,
        new_frame_uid,
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
        "Started recording time on tree {} at {}",
        ansi::format(&tree_name, ansi::ForestFormat::TreeName),
        ansi::format(
            &start_datetime.format("%H:%M").to_string(),
            ansi::ForestFormat::Time
        )
    );

    tree::switch(&tree_name).await?;

    Ok(())
}

/// Stops the current time recording(s)
///
/// # Errors
/// Returns an error if no recording were started
///
/// # Panics
/// This function may panic if database operations fail
pub async fn stop(datetime: Option<String>, create_note: bool) -> Result<(), Box<dyn Error>> {
    let stop_datetime = parse_user_datetime_or(&datetime, Local::now())?;

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
    let end_time = stop_datetime.timestamp_millis();
    let query_result = sqlx::query!(
        r#"
        UPDATE frame
        SET "end" = ?
        WHERE "end" is NULL;
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
        if create_note {
            // create a new note to write what was done in this work session
            notetaking::add(Some(frame.tree_name.clone()), true).await?;
        }

        let start_time: DateTime<Local> =
            DateTime::from_timestamp_millis(frame.start).unwrap().into();
        println!(
            "Stopped recording time on tree {}, started {} ({} {})",
            ansi::format(&frame.tree_name, ansi::ForestFormat::TreeName),
            ansi::format(&when(start_time), ansi::ForestFormat::Time),
            ansi::format(
                &start_time.format("%Y-%m-%d").to_string(),
                ansi::ForestFormat::Date
            ),
            ansi::format(
                &start_time.format("%H:%M").to_string(),
                ansi::ForestFormat::Time
            ),
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

    let current_tree_name = dbutils::get_current_tree_name(&pool).await?;

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
    println!(
        "On tree {}",
        ansi::format(&current_tree_name, ansi::ForestFormat::TreeName)
    );

    // print current time tracking recording if any
    match current_frame {
        Some(frame) => {
            let start_time: DateTime<Local> =
                DateTime::from_timestamp_millis(frame.start).unwrap().into();
            println!(
                "Recording time on tree {}, started {} ({} {})",
                ansi::format(&frame.tree_name, ansi::ForestFormat::TreeName),
                ansi::format(&when(start_time), ansi::ForestFormat::Time),
                ansi::format(
                    &start_time.format("%Y-%m-%d").to_string(),
                    ansi::ForestFormat::Date
                ),
                ansi::format(
                    &start_time.format("%H:%M").to_string(),
                    ansi::ForestFormat::Time
                ),
            );
        }
        None => println!("No recording started."),
    }

    Ok(())
}

/// Prints tree names and time spent on each
///
/// # Errors
/// Returns an error if the forest is empty
///
/// # Panics
/// This function may panic if database operations fail
pub async fn report(from: Option<String>, to: Option<String>) -> Result<(), Box<dyn Error>> {
    let pool = dbutils::load_db().await;

    let mut conn = pool
        .acquire()
        .await
        .expect("Acquiring connection to database should succeed");

    // get description of the desired tree
    let query_result = sqlx::query!(
        r#"
        SELECT MIN("start") AS earliest_start
        FROM frame;
        "#,
    )
    .fetch_one(&mut *conn)
    .await;

    // error handling
    let earliest_start_time = match query_result {
        Ok(record) => record,
        Err(query_error) => panic!("Database query failed: {query_error}"),
    };

    let from_datetime = parse_user_datetime_or(&from, DateTime::from(DateTime::<Local>::MIN_UTC))?;

    let to_datetime = parse_user_datetime_or(&to, DateTime::from(Local::now()))?;

    let from_ms = from_datetime.timestamp_millis();

    let to_ms = to_datetime.timestamp_millis();

    // get total time spent on every tree during the time interval
    let query_result = sqlx::query!(
        r#"
        -- get total time spent on each tree during the given time interval

        SELECT tree_name as name, SUM(min(coalesce(f."end", ?), ?) - max(f."start", ?)) AS total_time_spent
        FROM frame f
        RIGHT JOIN task t ON f.task_id = t.id
        WHERE
            f."start" BETWEEN ? AND ?
            OR f."end" BETWEEN ? AND ?
            OR ? between f."end" AND f."start"
        GROUP BY tree_name;
        "#,
        to_ms,
        to_ms,
        from_ms,
        from_ms,
        to_ms,
        from_ms,
        to_ms,
        from_ms,
    )
    .fetch_all(&mut *conn)
    .await;
    // error handling
    let records = match query_result {
        Ok(records) => records,
        Err(query_error) => panic!("Database query failed: {query_error}"),
    };

    println!(
        "From: {} {}",
        ansi::format(
            &from_datetime.format("%a %d %b %Y").to_string(),
            ansi::ForestFormat::Date
        ),
        ansi::format(
            &from_datetime.format("%H:%M").to_string(),
            ansi::ForestFormat::Time
        ),
    );
    println!(
        "To:   {} {}",
        ansi::format(
            &to_datetime.format("%a %d %b %Y").to_string(),
            ansi::ForestFormat::Date
        ),
        ansi::format(
            &to_datetime.format("%H:%M").to_string(),
            ansi::ForestFormat::Time
        ),
    );
    println!();
    // print tree names and time spent
    let mut total: TimeDelta = TimeDelta::milliseconds(0);
    for tree in records {
        let time_delta =
            TimeDelta::milliseconds(tree.total_time_spent.unwrap_or(0.0).round() as i64);
        total += time_delta;
        let hours = time_delta.num_hours();
        let minutes = time_delta.num_minutes() % 60;
        println!(
            "{} - {}h {}m",
            ansi::format(&tree.name, ansi::ForestFormat::TreeName),
            hours,
            minutes
        );
    }
    println!();

    // print total day time
    let hours = total.num_hours();
    let minutes = total.num_minutes() % 60;
    println!(
        "{} - {}h {}m",
        ansi::format("total", ansi::ForestFormat::TreeName),
        hours,
        minutes
    );

    Ok(())
}

pub async fn report_day() -> Result<(), Box<dyn Error>> {
    let midnight = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let from = Local::now()
        .with_time(midnight)
        .unwrap()
        .format("%H:%M")
        .to_string();
    let to = Local::now().format("%H:%M").to_string();
    report(Some(from), Some(to)).await
}

pub async fn report_week() -> Result<(), Box<dyn Error>> {
    let current_year = Local::now().year();
    let current_week = Local::now().iso_week().week();
    let midnight = NaiveTime::from_hms_opt(0, 0, 0).unwrap();

    let start_of_week = NaiveDate::from_isoywd_opt(current_year, current_week, Weekday::Mon).unwrap();
    let from = start_of_week
        .and_time(midnight)
        .format("%Y-%m-%d %H:%M")
        .to_string();
    let to = Local::now().format("%H:%M").to_string();
    report(Some(from), Some(to)).await
}

pub async fn report_month() -> Result<(), Box<dyn Error>> {
    let current_year = Local::now().year();
    let current_month = Local::now().month();
    let midnight = NaiveTime::from_hms_opt(0, 0, 0).unwrap();

    let start_of_month = NaiveDate::from_ymd_opt(current_year, current_month, 1).unwrap();
    let from = start_of_month
        .and_time(midnight)
        .format("%Y-%m-%d %H:%M")
        .to_string();
    let to = Local::now().format("%H:%M").to_string();
    report(Some(from), Some(to)).await
}
pub async fn report_year() -> Result<(), Box<dyn Error>> {
    let current_year = Local::now().year();
    let midnight = NaiveTime::from_hms_opt(0, 0, 0).unwrap();

    let start_of_month = NaiveDate::from_ymd_opt(current_year, 1, 1).unwrap();
    let from = start_of_month
        .and_time(midnight)
        .format("%Y-%m-%d %H:%M")
        .to_string();
    let to = Local::now().format("%H:%M").to_string();
    report(Some(from), Some(to)).await
}
