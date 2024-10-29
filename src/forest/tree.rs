use std::error::Error;

use super::ansi;
use super::dbutils;
use forest_types::{ListFormat, Uid};

/// Adds a tree to the forest
///
/// # Errors
/// Returns an error if a tree of this name already exists in the forest
///
/// # Panics
/// This function may panic if database operations fail
pub async fn add(name: String, description: String, edit: bool) -> Result<(), Box<dyn Error>> {
    let tree_description = if edit {
        edit::edit(description).expect("Could not open default editor")
    } else {
        description
    };

    let pool = dbutils::load_db().await;

    let mut conn = pool
        .acquire()
        .await
        .expect("Acquiring connection to database should succeed");

    // add new tree to tree table
    let query_result = sqlx::query!(
        r#"
        INSERT INTO tree("name", "description")
        VALUES (?, ?)
        "#,
        name,
        tree_description
    )
    .execute(&mut *conn)
    .await;

    // error handling
    match query_result {
        Ok(result) => {
            if result.rows_affected() != 1 {
                panic!("A single line should have been inserted into the tree table");
            }
        }
        Err(query_error) => match query_error {
            sqlx::Error::Database(db_error) => match db_error.kind() {
                sqlx::error::ErrorKind::UniqueViolation => {
                    return Err(
                        format!("A tree named '{name}' already exists in the forest.").into(),
                    )
                }
                _ => panic!("Database query failed: {db_error}"),
            },
            other_error => panic!("Database query failed: {other_error}"),
        },
    }

    let new_task_uid = Uid::new();

    // add new tree root to task table
    let query_result = sqlx::query!(
        r#"
        INSERT INTO task("id", "tree_name", "left", "right", "name", "description")
        VALUES (?, ?, 1, 2, "", "")
        "#,
        new_task_uid,
        name,
    )
    .execute(&mut *conn)
    .await;

    // error handling
    match query_result {
        Ok(result) => {
            if result.rows_affected() != 1 {
                panic!("A single line should have been inserted into the tree table");
            }
        }
        Err(query_error) => panic!("Database query failed: {query_error}"),
    }
    println!(
        "Added tree {}",
        ansi::format(&name, ansi::ForestFormat::TreeName)
    );

    // switch to new tree
    switch(&name).await?;

    Ok(())
}

/// Removes the given tree
///
/// # Errors
/// Returns an error if the given tree does not exist
///
/// # Panics
/// This function may panic if database operations fail
pub async fn remove(name: &String) -> Result<(), Box<dyn Error>> {
    let pool = dbutils::load_db().await;

    let mut conn = pool
        .acquire()
        .await
        .expect("Acquiring connection to database should succeed");

    // delete tree name from tree table
    // because of ON DELETE CASCADE constraint, all related tasks should be
    // removed from the task table too
    let query_result = sqlx::query!(
        r#"
        DELETE
        FROM tree
        WHERE name = ?
        "#,
        name,
    )
    .execute(&mut *conn)
    .await;

    // error handling
    match query_result {
        Ok(result) => {
            if result.rows_affected() != 1 {
                return Err(format!("Tree '{name}' not found").into());
            }
        }
        Err(query_error) => panic!("Database query failed: {query_error}"),
    }

    println!(
        "Removed tree {}",
        ansi::format(name, ansi::ForestFormat::TreeName)
    );

    Ok(())
}

/// Prints the current status of the forest
///
/// # Errors
/// Returns an error if the forest is empty
///
/// # Panics
/// This function may panic if database operations fail
pub async fn list(format: ListFormat) -> Result<(), Box<dyn Error>> {
    let pool = dbutils::load_db().await;

    let current_tree_name = dbutils::get_current_tree_name(&pool).await?;

    let mut conn = pool
        .acquire()
        .await
        .expect("Acquiring connection to database should succeed");

    // foreach tree get root task and first task if any
    let query_result = sqlx::query!(
        r#"
        -- foreach tree, get the root task and the first task if any

        SELECT tree_name, name, id, "left"
        FROM task 
        WHERE "left" BETWEEN 1 AND 2
        ORDER BY tree_name, "left" ASC;
        "#
    )
    .fetch_all(&mut *conn)
    .await;

    // error handling
    let tasks = match query_result {
        Ok(tasks) => tasks,
        Err(query_error) => panic!("Database query failed: {query_error}"),
    };

    // display depends on formatting config
    match format {
        ListFormat::Short => {
            for task in tasks {
                // in short formatting, only display tree names
                if task.left == 1 {
                    // identify current tree
                    if task.tree_name.eq(&current_tree_name) {
                        print!("* ");
                    } else {
                        print!("  ");
                    }
                    println!(
                        "{}",
                        ansi::format(&task.tree_name, ansi::ForestFormat::TreeName)
                    );
                    println!();
                }
            }
        }
        ListFormat::Long => {
            for task in tasks {
                // print tree name
                if task.left == 1 {
                    if task.tree_name.eq(&current_tree_name) {
                        print!("* ");
                    } else {
                        print!("  ");
                    }

                    println!(
                        "{}",
                        ansi::format(&task.tree_name, ansi::ForestFormat::TreeName)
                    );

                // print first tasks
                } else {
                    print!(
                        "    Next task: {} {}",
                        ansi::format(
                            Uid::try_from(task.id.clone()).unwrap().short(),
                            ansi::ForestFormat::TaskName
                        ),
                        ansi::format(&task.name, ansi::ForestFormat::TaskName)
                    );
                    print!(" ({})", ansi::format(&task.id, ansi::ForestFormat::Uid));
                    println!();
                    println!();
                }
            }
            println!();
        }
    }

    Ok(())
}

/// Prints the description of the given tree
///
/// # Errors
/// Returns an error if the given tree does not exist
///
/// # Panics
/// This function may panic if database operations fail
pub async fn show(name: &String) -> Result<(), Box<dyn Error>> {
    let pool = dbutils::load_db().await;

    let mut conn = pool
        .acquire()
        .await
        .expect("Acquiring connection to database should succeed");

    // get description of the desired tree
    let query_result = sqlx::query!(
        r#"
        SELECT name, description
        FROM tree
        WHERE name = ?;
        "#,
        name
    )
    .fetch_one(&mut *conn)
    .await;

    // error handling
    let record = match query_result {
        Ok(r) => r,
        Err(query_error) => match &query_error {
            sqlx::Error::RowNotFound => return Err(format!("Tree '{name}' not found").into()),
            _ => panic!("Database query failed: {query_error}"),
        },
    };

    println!(
        "tree {}",
        ansi::format(&record.name, ansi::ForestFormat::TreeName)
    );
    println!();

    for line in record.description.lines() {
        println!("    {line}");
    }

    Ok(())
}

/// Renames the given tree
///
/// # Errors
/// Returns an error if the given tree does not exist
///
/// # Panics
/// This function may panic if database operations fail
pub async fn rename(name: &String, new_name: String) -> Result<(), Box<dyn Error>> {
    let pool = dbutils::load_db().await;

    let mut conn = pool
        .acquire()
        .await
        .expect("Acquiring connection to database should succeed");

    // update name of the given tree
    let query_result = sqlx::query!(
        r#"
        UPDATE tree
        SET name = ?
        WHERE name = ?
        "#,
        new_name,
        name,
    )
    .execute(&mut *conn)
    .await;

    // error handling
    match query_result {
        Ok(result) => {
            if result.rows_affected() != 1 {
                return Err(format!("Tree '{name}' not found").into());
            }
        }
        Err(query_error) => panic!("Database query failed: {query_error}"),
    };

    println!(
        "Renamed tree {} to {}",
        ansi::format(name, ansi::ForestFormat::TreeName),
        ansi::format(&new_name, ansi::ForestFormat::TreeName)
    );

    Ok(())
}

/// Edit the description of the given tree
///
/// # Errors
/// Returns an error if the given tree does not exist
///
/// # Panics
/// This function may panic if database operations fail
pub async fn edit(name: &String) -> Result<(), Box<dyn Error>> {
    let pool = dbutils::load_db().await;

    let mut conn = pool
        .acquire()
        .await
        .expect("Acquiring connection to database should succeed");

    // get description of the given tree
    let query_result = sqlx::query!(
        r#"
        SELECT description
        FROM tree
        WHERE name = ?;
        "#,
        name
    )
    .fetch_one(&mut *conn)
    .await;

    // error handling
    let record = match query_result {
        Ok(r) => r,
        Err(query_error) => match &query_error {
            sqlx::Error::RowNotFound => return Err(format!("Tree '{name}' not found").into()),
            _ => panic!("Database query failed: {query_error}"),
        },
    };

    // edit description
    let description = edit::edit(record.description).expect("Could not open default editor");

    // update descripion of the given tree
    let query_result = sqlx::query!(
        r#"
        UPDATE tree
        SET description = ?
        WHERE name = ?
        "#,
        description,
        name,
    )
    .execute(&mut *conn)
    .await;

    // error handling
    match query_result {
        Ok(result) => {
            if result.rows_affected() != 1 {
                return Err(format!("Tree '{name}' not found").into());
            }
        }
        Err(query_error) => panic!("Database query failed: {query_error}"),
    };

    println!(
        "Edited description of tree {}",
        ansi::format(name, ansi::ForestFormat::TreeName)
    );

    Ok(())
}

/// Switch to a given
///
/// # Errors
/// Returns an error if the given tree does not exist
///
/// # Panics
/// This function may panic if database operations fail
pub async fn switch(name: &String) -> Result<(), Box<dyn Error>> {
    let pool = dbutils::load_db().await;

    let mut conn = pool
        .acquire()
        .await
        .expect("Acquiring connection to database should succeed");

    // udpate current_tree in state table
    let query_result = sqlx::query!(
        r#"
        UPDATE state
        SET current_tree = ?
        "#,
        name,
    )
    .execute(&mut *conn)
    .await;

    // error handling
    match query_result {
        Ok(_) => {}
        Err(query_error) => match query_error {
            sqlx::Error::Database(db_error) => match db_error.kind() {
                sqlx::error::ErrorKind::ForeignKeyViolation => {
                    return Err(format!("Tree '{name}' not found").into())
                }
                _ => panic!("Database query failed: {db_error}"),
            },
            _ => panic!("Database query failed: {query_error}"),
        },
    };

    println!(
        "Switched to tree {}",
        ansi::format(name, ansi::ForestFormat::TreeName)
    );

    Ok(())
}
