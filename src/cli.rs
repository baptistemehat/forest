use clap::{value_parser, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Forest")]
#[command(version = "0.0.1")]
#[command(about = "Forest - cli project manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Perform operations on tasks
    Task {
        #[command(subcommand)]
        command: TaskCommands,
    },

    /// Perform operations on trees
    Tree {
        #[command(subcommand)]
        command: TreeCommands,
    },

    /// Perform operations on notes
    Note {
        #[command(subcommand)]
        command: NoteCommands,
    },

    /// Switch to another tree
    Switch {
        /// Name of the tree to switch to
        #[arg(value_name = "NAME")]
        #[arg(value_parser = forest_types::tree_name_parser)]
        name: String,
    },

    /// Start recording time
    Start {
        /// Name of tree for which to record time
        #[arg(value_name = "TREE")]
        #[arg(value_parser = forest_types::tree_name_parser)]
        tree_name: Option<String>,

        /// Start date and time of recording
        #[arg(value_name = "DATETIME")]
        #[arg(long = "at", value_name = "FORMAT")]
        at: Option<String>,
    },

    /// Stop current time recording
    Stop {
        /// Stop date and time of recording
        #[arg(value_name = "DATETIME")]
        #[arg(long = "at", value_name = "FORMAT")]
        at: Option<String>,

        /// Do not create a new note
        #[arg(short = 'n', long = "no-note")]
        no_note: bool,
    },

    /// Show current time recording
    Status,

    /// Reports time spent on each tree
    Report {
        // /// Name of tree to get report of
        // #[arg(value_name = "TREE")]
        // #[arg(value_parser = forest_types::tree_name_parser)]
        // tree_name: Option<String>,

        /// 
        #[arg(short = 'f', long = "from", value_name = "DATETIME")]
        from: Option<String>,

        /// 
        #[arg(short = 't', long = "to", value_name = "DATETIME")]
        to: Option<String>,

        // #[arg(short = 'y', long = "year")]
        // year: bool,
        //
        // #[arg(short = 'm', long = "month")]
        // month: bool,
        // 
        // #[arg(short = 'w', long = "week")]
        // week: bool,

        #[arg(short = 'd', long = "day")]
        day: bool,
    },
}

#[derive(Subcommand)]
pub enum TaskCommands {
    /// List tasks in the current tree
    #[clap(alias = "ls")]
    List,
    /// Add a new task to the current tree
    Add {
        /// Name of the new task
        #[arg(value_name = "NAME")]
        #[arg(value_parser = forest_types::task_name_parser)]
        name: String,

        /// Uid of parent task. By default, adds the task to the tree root.
        #[arg(short = 'p', long = "parent", value_name = "UID")]
        #[arg(value_parser = forest_types::uid_parser)]
        parent_uid: Option<String>,

        /// Description of the new task
        #[arg(short = 'd', long = "description", value_name = "description")]
        description: Option<String>,

        /// Opens an editor to write the description of the new task
        #[arg(short = 'e', long = "edit")]
        edit: bool,
    },

    /// Remove a task from the current tree
    #[clap(alias = "rm")]
    Remove {
        /// Uid of the task
        #[arg(value_name = "UID")]
        #[arg(value_parser = forest_types::uid_parser)]
        uid: String,
    },

    /// Rename a task in the current tree
    Rename {
        /// Uid of the task
        #[arg(value_name = "UID")]
        #[arg(value_parser = forest_types::uid_parser)]
        uid: String,

        /// New name for the task
        #[arg(value_name = "NEW_NAME")]
        #[arg(value_parser = forest_types::task_name_parser)]
        new_name: String,
    },

    /// Show description of a task in the current tree
    Show {
        /// Uid of the task
        #[arg(value_name = "UID")]
        #[arg(value_parser = forest_types::uid_parser)]
        uid: String,
    },

    /// Edit description of a task in the current tree
    Edit {
        /// Uid of the task
        #[arg(value_name = "UID")]
        #[arg(value_parser = forest_types::uid_parser)]
        uid: String,
    },

    /// Set priority of a task in the current tree
    Priority {
        /// Uid of the task
        #[arg(value_name = "UID")]
        #[arg(value_parser = forest_types::uid_parser)]
        uid: String,

        /// Uid of the task
        #[arg(value_name = "PRIORITY")]
        #[arg(value_parser = value_parser!(forest_types::Priority))]
        priority: forest_types::Priority,
    },
}

#[derive(Subcommand)]
pub enum TreeCommands {
    /// List trees in the current forest
    #[clap(alias = "ls")]
    List {
        /// Formatting options
        #[arg(short = 'f', long = "format", value_name = "FORMAT")]
        format: Option<forest_types::ListFormat>,
    },

    /// Add a new tree
    Add {
        /// Name of the new tree
        #[arg(value_name = "NAME")]
        #[arg(value_parser = forest_types::tree_name_parser)]
        name: String,

        /// Description of the new tree
        #[arg(short = 'd', long = "description", value_name = "DESCRIPTION")]
        description: Option<String>,

        /// Opens an editor to write the description of the new tree
        #[arg(short = 'e', long = "edit")]
        edit: bool,
    },

    /// Remove a tree
    #[clap(alias = "rm")]
    Remove {
        /// Name of the tree
        #[arg(value_name = "NAME")]
        #[arg(value_parser = forest_types::tree_name_parser)]
        name: String,
    },

    /// Rename a tree
    Rename {
        /// Name of the tree
        #[arg(value_name = "NAME")]
        #[arg(value_parser = forest_types::tree_name_parser)]
        name: String,

        /// New name for the tree
        #[arg(value_name = "NEW_NAME")]
        #[arg(value_parser = forest_types::tree_name_parser)]
        new_name: String,
    },

    /// Show description of a tree
    Show {
        /// Name of the tree
        #[arg(value_name = "NAME")]
        #[arg(value_parser = forest_types::tree_name_parser)]
        name: String,
    },

    /// Edit description of a tree
    Edit {
        /// Name of the tree
        #[arg(value_name = "NAME")]
        #[arg(value_parser = forest_types::tree_name_parser)]
        name: String,
    },
}

#[derive(Subcommand)]
pub enum NoteCommands {
    /// List all notes
    #[clap(alias = "ls")]
    List {
        /// Show time tracking notes (hidden by default)
        #[arg(short = 't', long = "show-tt")]
        show_time_tracking: bool,
    },

    /// Create a new note associated to the current tree
    Add {
        /// Name of tree for which to add a note
        #[arg(value_name = "TREE")]
        #[arg(value_parser = forest_types::tree_name_parser)]
        tree_name: Option<String>,
    },

    /// Remove a note
    #[clap(alias = "rm")]
    Remove {
        /// Uid of the note
        #[arg(value_name = "UID")]
        #[arg(value_parser = forest_types::uid_parser)]
        uid: String,
    },

    /// Show content of a note
    Show {
        /// Uid of the note
        #[arg(value_name = "UID")]
        #[arg(value_parser = forest_types::uid_parser)]
        uid: String,
    },

    /// Edit a note
    Edit {
        /// Uid of the note
        #[arg(value_name = "UID")]
        #[arg(value_parser = forest_types::uid_parser)]
        uid: String,
    },
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
