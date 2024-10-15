use std::error::Error;

mod dbutils;
pub mod task;
pub mod timetracking;
pub mod tree;
pub mod types;

/// Prints the current status of the forest
///
/// # Errors
/// Returns an error if the forest is empty
///
/// # Panics
/// This function may panic if database operations fail
pub async fn report(show_uid: bool) -> Result<(), Box<dyn Error>> {
    let pool = dbutils::load_db().await;

    let current_tree_name =
        match dbutils::get_current_tree_name(&pool).await {
            Some(name) => name,
            None => return Err(
                "Nothing to display. It seems like your forest is empty.\nConsider adding a tree."
                    .into(),
            ),
        };

    let mut conn = pool
        .acquire()
        .await
        .expect("Acquiring connection to database should succeed");

    // get the first tasks of each tree
    let query_result = sqlx::query!(
        r#"
        -- select the first task of all trees (skipping the tree root 
        -- which is not a "task" per se)

        SELECT name, tree_name, id
        FROM task
        WHERE "left" = 2;
        "#
    )
    .fetch_all(&mut *conn)
    .await;

    // error handling
    let first_tasks = match query_result {
        Ok(tasks) => tasks,
        Err(query_error) => panic!("Database query failed: {query_error}"),
    };

    for task in first_tasks {
        // add a distinctive prefix before current tree
        if task.tree_name.eq(&current_tree_name) {
            print!("> ");
        } else {
            print!("  ");
        }

        // Print tree name and name of the first task
        println!("\x1b[1;38;5;0;48;5;2m{}\x1b[0m", task.tree_name);
        print!("    \x1b[1mNext task:\x1b[0m {}", task.name);

        if show_uid {
            print!(" ({})", task.id);
        }

        println!();
        println!();
    }

    Ok(())
}
