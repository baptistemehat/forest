use clap;
use nanoid::nanoid;
use std::char;
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
const SHORT_UID_LENGTH: usize = 7;

impl Uid {
    /// Constructs a new Unique identifier
    pub fn new() -> Self {
        Uid(nanoid!(UID_LENGTH, &UID_ALPHABET))
    }

    /// Returns the first characters of the UID
    pub fn short(&self) -> &str {
        &self.0[0..SHORT_UID_LENGTH]
    }
}

impl fmt::Display for Uid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for Uid {
    type Error = &'static str;

    /// Tries to construct a Uid from a String
    ///
    /// # Errors
    /// Returns an error if the length of the provided String if not the length of a Uid.
    /// Retruns an error if the provided String contains non-hex characters
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() != UID_LENGTH {
            Err("Length of provided String should be the same length as Uids")
        } else if !value
            .chars()
            .all(|c| c.is_ascii_hexdigit() && c.is_lowercase())
        {
            Err("Provided string should only contain lowercase hex characters")
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
    // uid should contain only hexadecimal characters
    if uid
        .chars()
        .all(|c| c.is_ascii_hexdigit() && c.is_lowercase())
    {
        Ok(uid.to_string())
    } else {
        Err(format!("'{uid}' is not a valid Uid. Uids can only contain lowercase hexadecimal characters (0-9a-z)"))
    }
}
