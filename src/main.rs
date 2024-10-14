use clap::{arg, value_parser, Arg, ArgAction, Command};
use std::process;

mod forest;

use forest::types;

#[tokio::main]
async fn main() {
    // clap cli parser
    let parser = Command::new("forest")
        .about("Forest - cli project manager")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands([

            // forest list
            Command::new("list")
                .alias("ls")
                .about("List tasks in the current tree")
                .arg(
                    Arg::new("show_uid")
                        .short('u')
                        .long("show-uid")
                        .action(ArgAction::SetTrue)
                        .help("Show task uids"),
                ),

            // forest add
            Command::new("add")
                .about("Add a new task to the current tree")
                .arg_required_else_help(true)
                .args(&[
                    arg!(name: <NAME> "name of the new task").value_parser(types::tree_name_parser),
                    arg!(parent: -p --parent <UID> "uid of parent task. By default, adds the task to the tree root.")
                        .value_parser(types::uid_parser),
                    arg!(description: -d --description <DESCRIPTION> "description of the new task"),
                    arg!(edit: -e --edit "opens an editor to write the description of the new task"),
                ]),

            // forest remove
            Command::new("remove")
                .alias("rm")
                .about("Remove a task from the current tree")
                .arg_required_else_help(true)
                .arg(arg!(uid: <UID> "uid of the task").value_parser(types::uid_parser)),

            // forest rename
            Command::new("rename")
                .about("Rename a task in the current tree")
                .arg_required_else_help(true)
                .args(&[
                    arg!(uid: <UID> "uid of the task").value_parser(types::uid_parser),
                    arg!(new_name: <NEW_NAME> "new name for the task").value_parser(types::task_name_parser),
                ]),

            // forest describe
            Command::new("describe")
                .about("Show description of a task in the current tree")
                .arg_required_else_help(true)
                .arg(arg!(uid: <UID> "uid of the task").value_parser(types::uid_parser)),

            // forest edit
            Command::new("edit")
                .about("Edit description of a task in the current tree")
                .arg_required_else_help(true)
                .arg(arg!(uid: <UID> "uid of the task").value_parser(types::uid_parser)),

            // forest priority
            Command::new("priority")
                .about("Set priority of a task in the current tree")
                .arg_required_else_help(true)
                .args([
                    arg!(uid: <UID> "uid of the task").value_parser(types::uid_parser),
                    arg!(priority: <PRIORITY> "priority value (integer, lower value is higher priority, starts at 1)")
                        .value_parser(value_parser!(types::Priority)),
                    ]),

            // forest switch
            Command::new("switch")
                .about("Switch to another tree")
                .arg_required_else_help(true)
                .arg(
                    arg!(name: <NAME> "name of the tree to switch to")
                    .value_parser(types::tree_name_parser),
                ),

            // forest status
            Command::new("status").about("Show forest status")
                .arg(Arg::new("show_uid")
                    .short('u')
                    .long("show-uid")
                    .action(ArgAction::SetTrue)
                    .help("Show tree and task uids")),

            // forest tree ...
            Command::new("tree")
                .about("Perform operations on trees")
                .visible_alias("project")
                .arg_required_else_help(true)
                .subcommands([

                    // forest tree list
                    Command::new("list").alias("ls").about("List all trees"),

                    // forest tree add
                    Command::new("add")
                        .about("Add a new tree")
                        .arg_required_else_help(true)
                        .args([
                            arg!(name: <NAME> "name of the new tree").value_parser(types::tree_name_parser),
                            arg!(description: -d --description <DESCRIPTION> "description of the tree"),
                            arg!(edit: -e --edit "opens an editor to write the description of the tree"),
                        ]),

                    // forest tree remove
                    Command::new("remove")
                        .about("Remove a tree")
                        .alias("rm")
                        .arg_required_else_help(true)
                        .arg(
                            arg!(name: <NAME> "name of the tree to remove")
                            .value_parser(types::tree_name_parser),
                        ),

                    // forest tree describe
                    Command::new("describe")
                        .about("Describe tree description")
                        .arg(
                            arg!(name: <NAME> "name of the tree to describe")
                            .value_parser(types::tree_name_parser),
                        ),

                    // forest tree rename
                    Command::new("rename")
                        .about("Rename a tree")
                        .arg_required_else_help(true)
                        .args(&[
                            arg!(name: <NAME> "name of the tree to rename")
                            .value_parser(types::tree_name_parser),
                            arg!(new_name: <NEW_NAME> "new name of the tree")
                            .value_parser(types::tree_name_parser),
                        ]),

                    // forest tree edit
                    Command::new("edit")
                        .about("Edit description of a tree")
                        .arg_required_else_help(true)
                        .arg(
                            arg!(name: <NAME> "name of the tree to edit")
                            .value_parser(types::tree_name_parser),
                        ),
                ]),
        ]);

    // process parsed input
    match parser.get_matches().subcommand() {
        // forest list
        Some(("list", list_matches)) => {
            let show_uid = list_matches.get_flag("show_uid");

            forest::task::list(show_uid).await.unwrap_or_else(|e| {
                eprintln!("list: {e}");
                process::exit(1);
            })
        }

        // forest add
        Some(("add", add_matches)) => {
            let task_name = match add_matches.get_one::<String>("name") {
                Some(name) => name,
                None => {
                    unreachable!("arg_required_else_help should make this code unreachable")
                }
            };

            let parent_uid = add_matches.get_one::<String>("parent");

            let edit = add_matches.get_flag("edit");

            let description = match add_matches.get_one::<String>("description") {
                Some(description) => description.clone(),
                None => String::from(""),
            };

            forest::task::add(task_name.clone(), parent_uid, description, edit)
                .await
                .unwrap_or_else(|e| {
                    eprintln!("add: {e}");
                    process::exit(1);
                })
        }

        // forest remove
        Some(("remove", remove_matches)) => {
            let task_uid = match remove_matches.get_one::<String>("uid") {
                Some(uid) => uid,
                None => {
                    unreachable!("arg_required_else_help should make this code unreachable")
                }
            };

            forest::task::remove(task_uid).await.unwrap_or_else(|e| {
                eprintln!("remove: {e}");
                process::exit(1);
            })
        }

        // forest rename
        Some(("rename", rename_matches)) => {
            let task_uid = match rename_matches.get_one::<String>("uid") {
                Some(uid) => uid,
                None => {
                    unreachable!("arg_required_else_help should make this code unreachable")
                }
            };

            let new_name = match rename_matches.get_one::<String>("new_name") {
                Some(name) => name,
                None => {
                    unreachable!("arg_required_else_help should make this code unreachable")
                }
            };

            forest::task::rename(task_uid, new_name.clone())
                .await
                .unwrap_or_else(|e| {
                    eprintln!("rename: {e}");
                    process::exit(1);
                })
        }

        // forest describe
        Some(("describe", describe_matches)) => {
            let task_uid = match describe_matches.get_one::<String>("uid") {
                Some(uid) => uid,
                None => {
                    unreachable!("arg_required_else_help should make this code unreachable")
                }
            };

            forest::task::describe(task_uid).await.unwrap_or_else(|e| {
                eprintln!("describe: {e}");
                process::exit(1);
            })
        }

        //  forest edit
        Some(("edit", edit_matches)) => {
            let task_uid = match edit_matches.get_one::<String>("uid") {
                Some(uid) => uid,
                None => {
                    unreachable!("arg_required_else_help should make this code unreachable")
                }
            };

            forest::task::edit(task_uid).await.unwrap_or_else(|e| {
                eprintln!("edit: {e}");
                process::exit(1);
            })
        }

        // forest priority
        Some(("priority", priority_matches)) => {
            let task_uid = match priority_matches.get_one::<String>("uid") {
                Some(uid) => uid,
                None => {
                    unreachable!("arg_required_else_help should make this code unreachable")
                }
            };

            let priority = match priority_matches.get_one::<types::Priority>("priority") {
                Some(priority) => priority,
                None => {
                    unreachable!("arg_required_else_help should make this code unreachable")
                }
            };

            forest::task::priority(task_uid, *priority)
                .await
                .unwrap_or_else(|e| {
                    eprintln!("priority: {e}");
                    process::exit(1);
                });
        }

        // forest switch
        Some(("switch", switch_matches)) => {
            let tree_name = match switch_matches.get_one::<String>("name") {
                Some(name) => name,
                None => {
                    unreachable!("arg_required_else_help should make this code unreachable")
                }
            };

            forest::tree::switch(tree_name).await.unwrap_or_else(|e| {
                eprintln!("switch: {e}");
                process::exit(1);
            });
        }

        // forest status
        Some(("status", status_matches)) => {
            let show_uid = status_matches.get_flag("show_uid");

            forest::status(show_uid).await.unwrap_or_else(|e| {
                eprintln!("status: {e}");
                process::exit(1);
            })
        }

        // forest tree ...
        Some(("tree", tree_matches)) => match tree_matches.subcommand() {
            // forest tree list
            Some(("list", _)) => {
                forest::tree::list().await;
            }

            // forest tree add
            Some(("add", tree_add_matches)) => {
                let tree_name = match tree_add_matches.get_one::<String>("name") {
                    Some(name) => name,
                    None => {
                        unreachable!("arg_required_else_help should make this code unreachable")
                    }
                };

                let edit = tree_add_matches.get_flag("edit");

                let description = match tree_add_matches.get_one::<String>("description") {
                    Some(description) => description.clone(),
                    None => String::from(""),
                };

                forest::tree::add(tree_name.clone(), description, edit)
                    .await
                    .unwrap_or_else(|e| {
                        eprintln!("tree add: {e}");
                        process::exit(1);
                    });
            }

            // forest tree remove
            Some(("remove", tree_rm_matches)) => {
                let tree_name = match tree_rm_matches.get_one::<String>("name") {
                    Some(name) => name,
                    None => {
                        unreachable!("arg_required_else_help should make this code unreachable")
                    }
                };

                forest::tree::remove(tree_name).await.unwrap_or_else(|e| {
                    eprintln!("tree rm: {e}");
                    process::exit(1);
                });
            }

            // forest tree describe
            Some(("describe", tree_describe_matches)) => {
                let tree_name = match tree_describe_matches.get_one::<String>("name") {
                    Some(name) => name,
                    None => {
                        unreachable!("arg_required_else_help should make this code unreachable")
                    }
                };

                forest::tree::describe(tree_name).await.unwrap_or_else(|e| {
                    eprintln!("tree show: {e}");
                    process::exit(1);
                });
            }

            // forest tree rename
            Some(("rename", tree_rename_matches)) => {
                let tree_name = match tree_rename_matches.get_one::<String>("name") {
                    Some(name) => name,
                    None => {
                        unreachable!("arg_required_else_help should make this code unreachable")
                    }
                };

                let new_name = match tree_rename_matches.get_one::<String>("new_name") {
                    Some(name) => name,
                    None => {
                        unreachable!("arg_required_else_help should make this code unreachable")
                    }
                };

                forest::tree::rename(tree_name, new_name.clone())
                    .await
                    .unwrap_or_else(|e| {
                        eprintln!("tree rename: {e}");
                        process::exit(1);
                    });
            }

            // forest tree edit
            Some(("edit", tree_edit_matches)) => {
                let tree_name = match tree_edit_matches.get_one::<String>("name") {
                    Some(name) => name,
                    None => {
                        unreachable!("arg_required_else_help should make this code unreachable")
                    }
                };

                forest::tree::edit(tree_name).await.unwrap_or_else(|e| {
                    eprintln!("tree edit: {e}");
                    process::exit(1);
                });
            }

            // invalid forest tree subcommand
            _ => unreachable!("arg_required_else_help should make this code unreachable"),
        },

        // // forest start
        // Some(("start", _)) => forest::start().unwrap_or_else(|e| {
        //     eprintln!("start: {e}");
        //     process::exit(1);
        // }),
        //
        // // forest stop
        // Some(("stop", _)) => forest::stop().unwrap_or_else(|e| {
        //     eprintln!("stop: {e}");
        //     process::exit(1);
        // }),
        //
        // // forest done
        // Some(("done", _)) => forest::done().unwrap_or_else(|e| {
        //     eprintln!("done: {e}");
        //     process::exit(1);
        // }),
        //
        // invalid forest subcommand
        _ => unreachable!("arg_required_else_help should make this code unreachable"),
    }
}
