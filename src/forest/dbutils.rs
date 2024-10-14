/// Name of config directory for the application
const FOREST_CONFIG_DIR: &str = "forest";

/// Name of db file storing user data
const SQLITE_DB_FILE_NAME: &str = "forest.db";

/// Creates the necessary SQLite tables if they do not exist
/// # Panics
/// This function may panic if the sql table creation fails
async fn create_tables_if_not_exist(pool: &sqlx::sqlite::SqlitePool) {
    let mut conn = pool
        .acquire()
        .await
        .expect("Acquiring connection to database should succeed");

    let _ = sqlx::query_file!("src/sql/create_tables.sql")
        .execute(&mut *conn)
        .await
        .expect("Should be able to create tables");
}

/// Tries to access database
/// Returns a `sqlx::Pool` on success.
/// # Panics
/// This function may panic if db file cannot be located or if connection to db failed
pub async fn load_db() -> sqlx::Pool<sqlx::Sqlite> {
    // get config directory
    let xdg_dirs = xdg::BaseDirectories::with_prefix(FOREST_CONFIG_DIR)
        .unwrap_or_else(|e| panic!("Cannot locate home directory: {e}"));

    // get path to database file
    let forest_path = match xdg_dirs.find_config_file(SQLITE_DB_FILE_NAME) {
        Some(path) => path,
        None => match xdg_dirs.place_config_file(SQLITE_DB_FILE_NAME) {
            Ok(path) => path,
            Err(e) => panic!("Cannot create forest store file: {e}"),
        },
    };

    // database connection options
    let options = sqlx::sqlite::SqliteConnectOptions::new()
        .filename(forest_path)
        .create_if_missing(true);

    // connect to database
    let pool = sqlx::sqlite::SqlitePool::connect_with(options)
        .await
        .expect("Connection to database should succeed in this context");

    create_tables_if_not_exist(&pool).await;

    pool
}

/// Returns the name of the current tree stored in db, if it exists. Otherwise returns the name of
/// another tree from the database.
/// Returns `None` if no current tree is found, ie. if the forest is empty.
///
/// # Panics
/// This function may panic if connection to db fails
pub async fn get_current_tree_name(pool: &sqlx::sqlite::SqlitePool) -> Option<String> {
    let mut conn = pool
        .acquire()
        .await
        .expect("Acquiring connection to database should succeed");

    let record_optional = sqlx::query!(
        r#"
        SELECT 
        CASE
            -- if a current tree is defined, return this current tree name
            WHEN s.current_tree IS NOT NULL THEN s.current_tree

            -- if no current tree is defined, return the first tree in tree table if any
            ELSE (SELECT t.name FROM tree t LIMIT 1)

            END AS current_tree
        FROM state s;
        "#
    )
    .fetch_optional(&mut *conn)
    .await
    .expect("Database query should succeed.");

    // match whether a line was returned or not
    match record_optional {
        // no line was returned from the query
        None => None,
        // a line was returned, ie. a tree name
        // record.current_tree is itself an option since its value in db can be null
        // since the column type in db does not prevent it to be null
        Some(record) => record.current_tree,
    }
}