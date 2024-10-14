use std::error::Error;

use super::dbutils;
use super::types;

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

    let new_task_uid = types::generate_uid();

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
    println!("Added tree '{}'", name);

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

    println!("Removed tree '{name}'");

    Ok(())
}

/// Lists all trees in the forest
///
/// # Panics
/// This function may panic if database operations fail
pub async fn list() {
    let pool = dbutils::load_db().await;

    let mut conn = pool
        .acquire()
        .await
        .expect("Acquiring connection to database should succeed");

    // get the name of all trees
    let query_result = sqlx::query!(r#"SELECT name FROM tree;"#)
        .fetch_all(&mut *conn)
        .await
        .expect("Database query should succeed");

    // print all tree names
    for tree in query_result {
        println!("   {}", tree.name);
    }
}

/// Prints the description of the given tree
///
/// # Errors
/// Returns an error if the given tree does not exist
///
/// # Panics
/// This function may panic if database operations fail
pub async fn describe(name: &String) -> Result<(), Box<dyn Error>> {
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

    println!();
    println!("\x1b[1;38;5;0;48;5;2m{}\x1b[0m", record.name);
    println!("{}", record.description);
    println!();

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

    println!("Renamed tree '{name}' to '{new_name}'");

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

    println!("Edited description of tree '{name}'");

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

    println!("Switched to tree '{name}'");

    Ok(())
}
