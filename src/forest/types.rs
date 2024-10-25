use clap;
use nanoid::nanoid;
use std::fmt;

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
#[derive(sqlx::Type, Clone)]
#[sqlx(transparent)]
pub struct Uid(String);

const UID_LENGTH: usize = 32;

impl Uid {
    pub fn new() -> Self {
        Uid(nanoid!(UID_LENGTH, &UID_ALPHABET))
    }
    pub fn short(&self) -> &str {
        &self.0[0..7]
    }
}

impl fmt::Display for Uid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for Uid {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() != UID_LENGTH {
            Err("Uid should be ")
        } else {
            Ok(Self(value))
        }
    }
}

/// UID alphabet
pub const UID_ALPHABET: [char; 16] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f',
];

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
pub fn uid_parser(uid: &str) -> Result<String, String> {
    Ok(uid.to_string())
}
