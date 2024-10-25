use std::error::Error;

use super::ansi;
use super::dbutils;
use super::types::{Priority, Uid};

async fn find_uid(short_uid: &String) -> Result<Uid, Box<dyn Error>> {
    let pool = dbutils::load_db().await;

    let current_tree_name = dbutils::get_current_tree_name(&pool).await?;

    let mut conn = pool
        .acquire()
        .await
        .expect("Acquiring connection to database should succeed");

    // get all uids from this current tree, that match the provided short uid, except for the root
    // task
    let query_result = sqlx::query!(
        r#"
        SELECT id, name
        FROM task
        WHERE tree_name = ? AND "left" != 1 AND id LIKE ? || '%';
        "#,
        current_tree_name,
        short_uid,
    )
    .fetch_all(&mut *conn)
    .await;

    match query_result {
        // Database error
        Err(query_error) => panic!("Database query failed: {query_error}"),

        // Query succeeded
        Ok(mut records) => {
            // if no task matching short uid was found
            if records.is_empty() {
                Err(format!("Task '{short_uid}' not found in tree '{current_tree_name}'").into())

                // if more than one task matches the short uid
            } else if records.len() > 1 {
                let mut error_message = format!("At least two tasks match '{short_uid}...':\n");
                for task in records {
                    error_message.push_str(&format!("- {}: {}\n", task.id, task.name));
                }
                error_message
                    .push_str("Please try to be more precise when refering to task uids\n");
                Err(error_message.into())

            // if the short uid only matched a single uid
            } else {
                let matching_record = records.pop().expect(
                    "There should be exactly one record in the records vector at this point",
                );

                Ok(Uid::try_from(matching_record.id)?)
            }
        }
    }
}

/// Adds a task to the current tree
///
/// # Errors
/// Returns an error if the parent uid does not exist in the current tree
///
/// # Panics
/// This function may panic if database operations fail
pub async fn add(
    name: String,
    parent_uid: Option<&String>,
    description: String,
    edit: bool,
) -> Result<(), Box<dyn Error>> {
    let pool = dbutils::load_db().await;

    let current_tree_name = dbutils::get_current_tree_name(&pool).await?;

    let mut conn = pool
        .acquire()
        .await
        .expect("Acquiring connection to database should succeed");

    let task_description = if edit {
        edit::edit(description).expect("Could not open default editor")
    } else {
        description
    };

    let parent_right = match parent_uid {
        Some(partial_uid) => {
            let uid = find_uid(partial_uid).await?;

            // get parent's "right" field
            let query_result = sqlx::query!(
                r#"
                SELECT "right", id, name
                FROM task
                WHERE tree_name = ? AND id = ?;
                "#,
                current_tree_name,
                uid,
            )
            .fetch_one(&mut *conn)
            .await;

            // error handling
            let parent_task = match query_result {
                Ok(record) => record,
                Err(query_error) => match query_error {
                    sqlx::Error::RowNotFound => {
                        return Err(
                            format!("Task '{uid}' not found in tree '{current_tree_name}'").into(),
                        )
                    }
                    other_error => panic!("Database query failed: {other_error}"),
                },
            };
            parent_task.right
        }
        None => {
            // if no parent_uid, parent is tree root

            // get tree root's "right" field
            let tree_root = sqlx::query!(
                r#"
                SELECT "right"
                FROM task
                WHERE tree_name = ? AND "left" = 1;
                "#,
                current_tree_name,
            )
            .fetch_one(&mut *conn)
            .await
            .expect("Should be able to get the root of the current tree");

            tree_root.right
        }
    };

    let new_task_uid = Uid::new();

    // check that new uid's short version is not in the db
    if find_uid(&new_task_uid.short().to_string()).await.is_ok() {
        panic!("Birthday paradox hit");
    }
    // update position of all tasks at the right of the parent
    let query_result = sqlx::query!(
        r#"
        UPDATE task
        SET 
            -- all tasks at the right of the inserted task should be shifted right
            -- by the width of the task, ie. 2

            "left" = CASE WHEN "left" > ? THEN "left" + 2 ELSE "left" END,
            "right" = CASE WHEN "right" >= ? THEN "right" + 2 ELSE "right" END

        WHERE "right" >= ? AND tree_name = ?;
        "#,
        parent_right,
        parent_right,
        parent_right,
        current_tree_name
    )
    .execute(&mut *conn)
    .await;

    // error handling
    match query_result {
        Ok(result) => {
            if result.rows_affected() < 1 {
                panic!("Insertion should at least update one row (root)");
            }
        }
        Err(query_error) => panic!("Database query failed: {query_error}"),
    };

    // insert the new task
    let new_task_left = parent_right;
    let new_task_right = parent_right + 1;
    let query_result = sqlx::query!(
        r#"
        INSERT INTO task("id", "tree_name", "left", "right", "name", "description")
        VALUES(?, ?, ?, ?, ?, ?);
        "#,
        new_task_uid,
        current_tree_name,
        new_task_left,
        new_task_right,
        name,
        task_description,
    )
    .execute(&mut *conn)
    .await;

    // error handling
    match query_result {
        Ok(result) => {
            if result.rows_affected() != 1 {
                panic!("A single line should have been inserted into the task table");
            }
        }
        Err(query_error) => match query_error {
            sqlx::Error::Database(db_error) => match db_error.kind() {
                sqlx::error::ErrorKind::UniqueViolation => {
                    panic!("Task id should be unique");
                }
                _ => panic!("Database query failed: {db_error}"),
            },
            other_error => panic!("Database query failed: {other_error}"),
        },
    }

    println!(
        "Added task {} ({}) to tree {}",
        ansi::format(&name, ansi::ForestFormat::TaskName),
        ansi::format(new_task_uid.short(), ansi::ForestFormat::Uid),
        ansi::format(&current_tree_name, ansi::ForestFormat::TreeName)
    );

    Ok(())
}

/// Removes a task from the current tree
///
/// # Errors
/// Returns an error if the task does not exist in the current tree
///
/// # Panics
/// This function may panic if database operations fail
pub async fn remove(partial_uid: &String) -> Result<(), Box<dyn Error>> {
    let pool = dbutils::load_db().await;

    let current_tree_name = dbutils::get_current_tree_name(&pool).await?;

    let mut conn = pool
        .acquire()
        .await
        .expect("Acquiring connection to database should succeed");

    let uid = find_uid(partial_uid).await?;

    // get left and right values of the task we want to remove
    // this information is neede later to shift remaining tasks to fill the gap resulting from the
    // removal
    let query_result = sqlx::query!(
        r#"
        SELECT "right", "left", "name"
        FROM task
        WHERE tree_name = ? AND id = ?;
        "#,
        current_tree_name,
        uid,
    )
    .fetch_one(&mut *conn)
    .await;

    let task = match query_result {
        Ok(record) => record,
        Err(query_error) => match query_error {
            sqlx::Error::RowNotFound => {
                return Err(format!("Task '{uid}' not found in tree '{current_tree_name}'").into())
            }
            other_error => panic!("Database query failed: {other_error}"),
        },
    };

    // remove the task and all subtasks
    let removed_task_left = task.left;
    let removed_task_right = task.right;
    let shift = task.right - task.left + 1;
    let query_result = sqlx::query!(
        r#"
        DELETE FROM task
        WHERE 
            tree_name = ? AND
            "left" BETWEEN ? AND ?;
        "#,
        current_tree_name,
        removed_task_left,
        removed_task_right
    )
    .execute(&mut *conn)
    .await;

    match query_result {
        Ok(result) => {
            if result.rows_affected() < 1 {
                return Err(format!("Task '{uid}' not found").into());
            }
        }
        Err(query_error) => panic!("Database query failed: {query_error}"),
    }

    // update position of all tasks at the right of the removed task to fill the gaps created by
    // the removal
    let query_result = sqlx::query!(
        r#"
        UPDATE task
        SET 
            -- all tasks at the right of the removed task should be shifted left
            -- by the width of the removed task (subtree if it has subtasks)

            "left" = CASE WHEN "left" > ? THEN "left" - ? ELSE "left" END,
            "right" = CASE WHEN "right" > ? THEN "right" - ? ELSE "right" END
        WHERE tree_name = ?;
        "#,
        removed_task_right,
        shift,
        removed_task_right,
        shift,
        current_tree_name,
    )
    .execute(&mut *conn)
    .await;

    // error handling
    match query_result {
        Ok(result) => {
            if result.rows_affected() < 1 {
                panic!("Removal should at least update one row (root)");
            }
        }
        Err(query_error) => panic!("Database query failed: {query_error}"),
    };

    println!(
        "Removed task {} ({}) from tree {}",
        ansi::format(&task.name, ansi::ForestFormat::TaskName),
        ansi::format(uid.short(), ansi::ForestFormat::Uid),
        ansi::format(&current_tree_name, ansi::ForestFormat::TreeName)
    );

    Ok(())
}

/// Renames a task from the current tree
pub async fn rename(partial_uid: &String, name: String) -> Result<(), Box<dyn Error>> {
    let pool = dbutils::load_db().await;

    let current_tree_name = dbutils::get_current_tree_name(&pool).await?;

    let mut conn = pool
        .acquire()
        .await
        .expect("Acquiring connection to database should succeed");

    let uid = find_uid(partial_uid).await?;

    // get the current name of the  task to rename from the task table
    // we need to retrieve this for output message
    let query_result = sqlx::query!(
        r#"
        SELECT name 
        FROM task
        WHERE tree_name = ? AND id LIKE ? || '%';
        "#,
        current_tree_name,
        uid,
    )
    .fetch_one(&mut *conn)
    .await;

    // error handling
    let task = match query_result {
        Ok(record) => record,
        Err(query_error) => match query_error {
            sqlx::Error::RowNotFound => {
                return Err(format!("Task '{uid}' not found in tree '{current_tree_name}'").into())
            }
            other_error => panic!("Database query failed: {other_error}"),
        },
    };

    // update name of the task to rename
    let query_result = sqlx::query!(
        r#"
        UPDATE task
        SET name = ?
        WHERE tree_name = ? AND id LIKE ? || '%';
        "#,
        name,
        current_tree_name,
        uid,
    )
    .execute(&mut *conn)
    .await;

    // error handling
    match query_result {
        Ok(result) => {
            if result.rows_affected() != 1 {
                panic!("Renaming should at least update one row (the task to rename)");
            }
        }
        Err(query_error) => panic!("Database query failed: {query_error}"),
    };

    println!(
        "Renamed task {} ({}) to {}",
        ansi::format(&task.name, ansi::ForestFormat::TaskName),
        ansi::format(uid.short(), ansi::ForestFormat::Uid),
        ansi::format(&name, ansi::ForestFormat::TaskName)
    );

    Ok(())
}

/// Edits a task in the current tree
pub async fn edit(partial_uid: &String) -> Result<(), Box<dyn Error>> {
    let pool = dbutils::load_db().await;

    let current_tree_name = dbutils::get_current_tree_name(&pool).await?;

    let mut conn = pool
        .acquire()
        .await
        .expect("Acquiring connection to database should succeed");

    let uid = find_uid(partial_uid).await?;

    // get name and description of the task to edit
    let query_result = sqlx::query!(
        r#"
        SELECT name, description
        FROM task
        WHERE tree_name = ? AND id LIKE ? || '%';
        "#,
        current_tree_name,
        uid,
    )
    .fetch_one(&mut *conn)
    .await;

    // error handling
    let task = match query_result {
        Ok(record) => record,
        Err(query_error) => match query_error {
            sqlx::Error::RowNotFound => {
                return Err(format!("Task '{uid}' not found in tree '{current_tree_name}'").into())
            }
            other_error => panic!("Database query failed: {other_error}"),
        },
    };

    // open default editor and edit current description
    let description = edit::edit(task.description).expect("Could not open default editor");

    // update description of the task
    let query_result = sqlx::query!(
        r#"
        UPDATE task
        SET description = ?
        WHERE tree_name = ? AND id LIKE ? || '%';
        "#,
        description,
        current_tree_name,
        uid,
    )
    .execute(&mut *conn)
    .await;

    // error handling
    match query_result {
        Ok(result) => {
            if result.rows_affected() != 1 {
                panic!("Editing should at least update one row (the task to edit)");
            }
        }
        Err(query_error) => panic!("Database query failed: {query_error}"),
    };

    println!(
        "Edited description of task {} ({})",
        ansi::format(&task.name, ansi::ForestFormat::TaskName),
        ansi::format(uid.short(), ansi::ForestFormat::Uid),
    );

    Ok(())
}

/// Lists all tasks in the current tree
pub async fn list() -> Result<(), Box<dyn Error>> {
    let pool = dbutils::load_db().await;

    let current_tree_name = dbutils::get_current_tree_name(&pool).await?;

    let mut conn = pool
        .acquire()
        .await
        .expect("Acquiring connection to database should succeed");

    // get all tasks of a tree, order by ascending "left"
    let task_vec = sqlx::query!(
        r#"
        SELECT id, name, "right", "left"
        FROM task
        WHERE tree_name = ?
        ORDER BY "left" ASC;
        "#,
        current_tree_name,
    )
    .fetch_all(&mut *conn)
    .await
    .expect("Should be able to get tasks from the tree");

    // this stack will hold the "right" field of the ancestor tasks as we traverse the tree
    // it is used to print indent characters
    let mut stack: Vec<i64> = Vec::new();

    // print tree name as a header
    println!(
        "{}",
        ansi::format(&current_tree_name, ansi::ForestFormat::TreeName)
    );

    let mut task_iter = task_vec.iter();

    // the first "task" of the tree is not really a task per se
    // it just represents the root of the tree, meaning all tasks of the tree are descendants of
    // this root "task"
    //
    // so here, we push its "right" value to the stack for later indenting and we "skip" it with
    // the "next" method.
    stack.push(task_iter.next().unwrap().right);

    for task in task_iter {
        // if the current task is "out" of the boundaries of previous parent tasks, it means we
        // processed all descendants of these parent tasks, and are now processing "siblings" or
        // ancestors of these parent tasks
        // therefore, we can pop them out from the stack
        while task.left > *stack.last().unwrap() {
            stack.pop();
        }

        // print indent based on the number of ancestors of the current task
        //
        // foreach ancestor in the stack (starting at the root and descending to the current task)
        // check whether each ancestor is the younger (ie. last) child of its own parent or not.
        let mut stack_iter = stack.iter().peekable();
        while let Some(parent) = stack_iter.next() {
            if let Some(&child) = stack_iter.peek() {
                // if ancestor is youngest child of its parent, print an empty indent
                if *child == parent - 1 {
                    print!("  ");
                } else {
                    print!(
                        "{} ",
                        ansi::format(&String::from("│"), ansi::ForestFormat::Box)
                    );
                }
            }
        }

        // if current task is the youngest (ie. last) child of its parent
        // "stop" the vertical line here
        if task.right == (stack.last().unwrap() - 1) {
            print!(
                "{}",
                ansi::format(&String::from("└╴"), ansi::ForestFormat::Box),
            );
        } else {
            print!(
                "{}",
                ansi::format(&String::from("├╴"), ansi::ForestFormat::Box),
            );
        }
        print!(
            "{} {}",
            ansi::format(
                Uid::try_from(task.id.clone()).unwrap().short(),
                ansi::ForestFormat::Uid
            ),
            ansi::format(&task.name, ansi::ForestFormat::TaskName)
        );

        println!();

        // if task is a parent (ie. its width is higher than 1), then push its "right" value to the
        // stack to increment the indent of its children, which are the task we will process in the
        // next loop cycle
        let width = task.right - task.left;
        if width > 1 {
            stack.push(task.right);
        }
    }
    Ok(())
}

/// Shows the description of a task in the current tree
pub async fn show(partial_uid: &String) -> Result<(), Box<dyn Error>> {
    let pool = dbutils::load_db().await;

    let current_tree_name = dbutils::get_current_tree_name(&pool).await?;

    let mut conn = pool
        .acquire()
        .await
        .expect("Acquiring connection to database should succeed");

    let uid = find_uid(partial_uid).await?;

    // get description of the desired task
    let query_result = sqlx::query!(
        r#"
        SELECT name, description
        FROM task
        WHERE tree_name = ? AND id LIKE ? || '%';
        "#,
        current_tree_name,
        uid,
    )
    .fetch_one(&mut *conn)
    .await;

    // error handling
    let task = match query_result {
        Ok(record) => record,
        Err(query_error) => match query_error {
            sqlx::Error::RowNotFound => {
                return Err(format!("Task '{uid}' not found in tree '{current_tree_name}'").into())
            }
            other_error => panic!("Database query failed: {other_error}"),
        },
    };

    println!(
        "task {}",
        ansi::format(&uid.to_string(), ansi::ForestFormat::Uid)
    );
    println!(
        "Tree: {}",
        ansi::format(&current_tree_name, ansi::ForestFormat::TreeName)
    );
    println!(
        "Name: {}",
        ansi::format(&task.name, ansi::ForestFormat::TaskName)
    );
    println!();

    for line in task.description.lines() {
        println!("    {line}");
    }

    Ok(())
}

/// Sets the priority of a task in the current tree
pub async fn priority(partial_uid: &String, priority: Priority) -> Result<(), Box<dyn Error>> {
    let pool = dbutils::load_db().await;

    let current_tree_name = dbutils::get_current_tree_name(&pool).await?;

    let mut conn = pool
        .acquire()
        .await
        .expect("Acquiring connection to database should succeed");

    let uid = find_uid(partial_uid).await?;

    // get left and right boundaries of the task/subtree to move
    let query_result = sqlx::query!(
        r#"
        SELECT "left", "right", "name"
        FROM task
        WHERE tree_name = ? AND id = ?;
        "#,
        current_tree_name,
        uid,
    )
    .fetch_one(&mut *conn)
    .await;

    // error handling
    let moved_task = match query_result {
        Ok(record) => record,
        Err(query_error) => match query_error {
            sqlx::Error::RowNotFound => {
                return Err(format!("Task '{uid}' not found in tree '{current_tree_name}'").into())
            }
            other_error => panic!("Database query failed: {other_error}"),
        },
    };

    // get all ascendants of the task to move and ony select the one parent with the highest "left"
    // boundary, ie. the immediate parent of the task to move
    let moved_task_left = moved_task.left;
    let moved_task_right = moved_task.right;
    let query_result = sqlx::query!(
        r#"
        SELECT "left", "right", "id"
        FROM task
        WHERE 
            tree_name = ? AND
            "left" < ? AND ? < "right"
        ORDER BY "left" DESC
        LIMIT 1;
        "#,
        current_tree_name,
        moved_task_left,
        moved_task_right,
    )
    .fetch_one(&mut *conn)
    .await;

    // error handling
    let parent_task = match query_result {
        Ok(record) => record,
        Err(query_error) => match query_error {
            sqlx::Error::RowNotFound => {
                panic!("Getting parent task should succeed here since there exist at least one ascendant task (root)");
            }
            other_error => panic!("Database query failed: {other_error}"),
        },
    };

    // get the nth child of the parent task, ie. the "position" we want to move the task to
    let parent_task_left = parent_task.left;
    let parent_task_right = parent_task.right;
    let parent_task_id = parent_task.id;
    let child_index: u32 = u32::try_from(priority - 1).unwrap();
    let query_result = sqlx::query!(
        r#"
        SELECT "right", "left", "id"
        FROM task AS child
        WHERE 
            -- get all immediate children of the parent task
            -- ie. get all descendants of the parent task...

            tree_name = ? 
            AND child."left" BETWEEN ? AND ?
            AND child."id" IS NOT ?

            -- ... for which there is NO "middle task" that would be a descendant of the parent
            -- task and a parent of the descendant task

            AND NOT EXISTS
            (SELECT *
             FROM task AS mid_task
             WHERE 
                -- try to find tasks that are both descendant of the parent task, and parent of
                -- the descendant task

                tree_name = ?
                AND mid_task."left" BETWEEN ? AND ?
                AND child."left" BETWEEN mid_task."left"  AND  mid_task."right"
                AND mid_task.id NOT IN (?, child.id)
             )

        -- only select the nth child

        ORDER BY "left" ASC
        LIMIT 1 OFFSET ?;
        "#,
        current_tree_name,
        parent_task_left,
        parent_task_right,
        parent_task_id,
        current_tree_name,
        parent_task_left,
        parent_task_right,
        parent_task_id,
        child_index,
    )
    .fetch_one(&mut *conn)
    .await;

    // error handling
    let targetted_child_task = match query_result {
        Ok(record) => record,
        Err(query_error) => match query_error {
            sqlx::Error::RowNotFound => {
                panic!("Getting the nth child should succeed since there is at least one child in the tree (the task to be moved)");
            }
            other_error => panic!("Database query failed: {other_error}"),
        },
    };

    let shift = moved_task.right - moved_task.left + 1;
    let moved_task_right = moved_task.right;
    let wanted_position_left = targetted_child_task.left;
    let wanted_position_right = targetted_child_task.right;

    // if the task is to be moved to its left
    if wanted_position_left < moved_task.left {
        // move the task and its descendants to their left
        // and update surronding tasks to fill the gaps
        let query_result = sqlx::query!(
            r#"
            UPDATE task
            SET
                "left" = 
                    CASE 
                        -- if the task is the task to move or one of its descendants
                        -- then move it to its left to the wanted position
                        WHEN "left" BETWEEN ? AND ? THEN "left" - (? - ?)

                        -- if the task is between the task to move and the wanted position
                        -- then shift it to its right by the width of the moved task
                        WHEN "left" BETWEEN ? AND ?-1 THEN "left" + ?

                        -- else, do not move
                        ELSE "left" END,

                "right" = 
                    CASE 
                        -- if the task is the task to move or one of its descendants
                        -- then move it to its left to the wanted position
                        WHEN "right" BETWEEN ? AND ? THEN "right" - (? - ?)

                        -- if the task is between the task to move and the wanted position
                        -- then shift it to its right by the width of the moved task
                        WHEN "right" BETWEEN ? AND ?-1 THEN "right" + ?

                        -- else, do not move
                        ELSE "right" END
            WHERE tree_name = ?
            "#,
            moved_task_left,
            moved_task_right,
            moved_task_left,
            wanted_position_left,
            wanted_position_left,
            moved_task_left,
            shift,
            moved_task_left,
            moved_task_right,
            moved_task_left,
            wanted_position_left,
            wanted_position_left,
            moved_task_left,
            shift,
            current_tree_name,
        )
        .execute(&mut *conn)
        .await;

        // error handling
        match query_result {
            Ok(result) => {
                if result.rows_affected() < 1 {
                    panic!("Moving tasks should at least update one row (the task movrd)");
                }
            }
            Err(query_error) => panic!("Database query failed: {query_error}"),
        };
    }
    // if the task is to be moved to its right
    else if moved_task.right < wanted_position_right {
        // move the task and its descendants to their right
        // and update surronding tasks to fill the gaps
        let query_result = sqlx::query!(
            r#"
            UPDATE task
            SET
                "left" = 
                    CASE 
                        -- if the task is the task to move or one of its descendants
                        -- then move it to its right to the wanted position
                        WHEN "left" BETWEEN ? AND ? THEN "left" + (? - ?)

                        -- if the task is between the task to move and the wanted position
                        -- then shift it to its left by the width of the moved task
                        WHEN "left" BETWEEN ?+1 AND ? THEN "left" - ?

                        -- else, do not move
                        ELSE "left" END,
                "right" = 
                    CASE 
                        -- if the task is the task to move or one of its descendants
                        -- then move it to its right to the wanted position
                        WHEN "right" BETWEEN ? AND ? THEN "right" + (? - ?)

                        -- if the task is between the task to move and the wanted position
                        -- then shift it to its left by the width of the moved task
                        WHEN "right" BETWEEN ?+1 AND ? THEN "right" -?

                        -- else, do not move
                        ELSE "right" END
            WHERE tree_name = ?
            "#,
            moved_task_left,
            moved_task_right,
            wanted_position_right,
            moved_task_right,
            moved_task_right,
            wanted_position_right,
            shift,
            moved_task_left,
            moved_task_right,
            wanted_position_right,
            moved_task_right,
            moved_task_right,
            wanted_position_right,
            shift,
            current_tree_name,
        )
        .execute(&mut *conn)
        .await;

        // error handling
        match query_result {
            Ok(result) => {
                if result.rows_affected() < 1 {
                    panic!("Moving tasks should at least update one row (the task movrd)");
                }
            }
            Err(query_error) => panic!("Database query failed: {query_error}"),
        };
    }

    println!(
        "Changed priority of task {} ({})",
        ansi::format(&moved_task.name, ansi::ForestFormat::TaskName),
        ansi::format(uid.short(), ansi::ForestFormat::Uid),
    );

    Ok(())
}
