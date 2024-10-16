use clap;
use nanoid::nanoid;

/// Possible formatting for `list` commands
#[derive(clap::ValueEnum, Clone, Default)]
pub enum ListFormat {
    /// only display tree names
    #[default]
    Short,

    // display tree names, and next task
    Long,
}

/// Unique Identifier
pub type Uid = String;

/// UID alphabet
pub const UID_ALPHABET: [char; 16] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f',
];

/// Returns a new uid
pub fn generate_uid() -> Uid {
    nanoid!(32, &UID_ALPHABET)
}

/// Priority of tasks.
/// High priority is expressed with low integers:
/// * `10` is a higher priority than `11`
/// * `0` is highest priority
pub type Priority = u64;

/// Parses a tree name
pub fn tree_name_parser(tree_name: &str) -> Result<String, String> {
    Ok(tree_name.to_string())
}

/// Parses a task name
pub fn task_name_parser(task_name: &str) -> Result<String, String> {
    Ok(task_name.to_string())
}

/// Parses a uid
pub fn uid_parser(uid: &str) -> Result<Uid, String> {
    Ok(uid.to_string())
}
