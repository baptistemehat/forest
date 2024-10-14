use crate::forest::types;
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
    /// List tasks in the current tree
    #[clap(alias = "ls")]
    List {
        /// Show task uids
        #[arg(short = 'u', long = "show-uid")]
        show_uid: bool,
    },

    /// Add a new task to the current tree
    Add {
        /// Name of the new task
        #[arg(value_name = "NAME")]
        #[arg(value_parser = types::task_name_parser)]
        name: String,

        /// Uid of parent task. By default, adds the task to the tree root.
        #[arg(short = 'p', long = "parent", value_name = "UID")]
        #[arg(value_parser = types::uid_parser)]
        parent_uid: Option<types::Uid>,

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
        #[arg(value_parser = types::uid_parser)]
        uid: types::Uid,
    },

    /// Rename a task in the current tree
    Rename {
        /// Uid of the task
        #[arg(value_name = "UID")]
        #[arg(value_parser = types::uid_parser)]
        uid: types::Uid,

        /// New name for the task
        #[arg(value_name = "NEW_NAME")]
        #[arg(value_parser = types::task_name_parser)]
        new_name: String,
    },

    /// Show description of a task in the current tree
    Describe {
        /// Uid of the task
        #[arg(value_name = "UID")]
        #[arg(value_parser = types::uid_parser)]
        uid: types::Uid,
    },

    /// Edit description of a task in the current tree
    Edit {
        /// Uid of the task
        #[arg(value_name = "UID")]
        #[arg(value_parser = types::uid_parser)]
        uid: types::Uid,
    },

    /// Set priority of a task in the current tree
    Priority {
        /// Uid of the task
        #[arg(value_name = "UID")]
        #[arg(value_parser = types::uid_parser)]
        uid: types::Uid,

        /// Uid of the task
        #[arg(value_name = "PRIORITY")]
        #[arg(value_parser = value_parser!(types::Priority))]
        priority: types::Priority,
    },

    /// Perform operations on trees
    Tree {
        #[command(subcommand)]
        command: TreeCommands,
    },

    /// Switch to another tree
    Switch {
        /// Name of the tree to switch to
        #[arg(value_name = "NAME")]
        #[arg(value_parser = types::tree_name_parser)]
        name: String,
    },

    /// Show forest status
    Status {
        /// Show task uids
        #[arg(short = 'u', long = "show-uid")]
        show_uid: bool,
    },
}

#[derive(Subcommand)]
pub enum TreeCommands {
    /// List trees in the current forest
    #[clap(alias = "ls")]
    List,

    /// Add a new tree
    Add {
        /// Name of the new tree
        #[arg(value_name = "NAME")]
        #[arg(value_parser = types::tree_name_parser)]
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
        /// Name of the new tree
        #[arg(value_name = "NAME")]
        #[arg(value_parser = types::tree_name_parser)]
        name: String,
    },

    /// Rename a tree
    Rename {
        /// Name of the tree
        #[arg(value_name = "NAME")]
        #[arg(value_parser = types::tree_name_parser)]
        name: String,

        /// New name for the tree
        #[arg(value_name = "NEW_NAME")]
        #[arg(value_parser = types::tree_name_parser)]
        new_name: String,
    },

    /// Show description of a tree
    Describe {
        /// Name of the tree
        #[arg(value_name = "NAME")]
        #[arg(value_parser = types::tree_name_parser)]
        name: String,
    },

    /// Edit description of a tree
    Edit {
        /// Name of the tree
        #[arg(value_name = "NAME")]
        #[arg(value_parser = types::tree_name_parser)]
        name: String,
    },
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}