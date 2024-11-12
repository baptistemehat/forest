use clap::Parser;
use std::process;

mod cli;
mod forest;

#[tokio::main]
async fn main() {
    let cli_parser = cli::Cli::parse();

    match cli_parser.command {
        cli::Commands::Task { command } => match command {
            cli::TaskCommands::List => forest::task::list().await.unwrap_or_else(|e| {
                eprintln!("list: {e}");
                process::exit(1);
            }),

            cli::TaskCommands::Add {
                name,
                parent_uid,
                description,
                edit,
            } => forest::task::add(
                name,
                parent_uid.as_ref(),
                description.unwrap_or_default(),
                edit,
            )
            .await
            .unwrap_or_else(|e| {
                eprintln!("add: {e}");
                process::exit(1);
            }),

            cli::TaskCommands::Remove { uid } => {
                forest::task::remove(&uid).await.unwrap_or_else(|e| {
                    eprintln!("remove: {e}");
                    process::exit(1);
                });
            }

            cli::TaskCommands::Rename { uid, new_name } => {
                forest::task::rename(&uid, new_name)
                    .await
                    .unwrap_or_else(|e| {
                        eprintln!("rename: {e}");
                        process::exit(1);
                    });
            }

            cli::TaskCommands::Show { uid } => {
                forest::task::show(&uid).await.unwrap_or_else(|e| {
                    eprintln!("show: {e}");
                    process::exit(1);
                });
            }

            cli::TaskCommands::Edit { uid } => {
                forest::task::edit(&uid).await.unwrap_or_else(|e| {
                    eprintln!("edit: {e}");
                    process::exit(1);
                });
            }

            cli::TaskCommands::Priority { uid, priority } => {
                forest::task::priority(&uid, priority)
                    .await
                    .unwrap_or_else(|e| {
                        eprintln!("priority: {e}");
                        process::exit(1);
                    });
            }
        },
        cli::Commands::Tree { command } => match command {
            cli::TreeCommands::List { format } => {
                forest::tree::list(format.unwrap_or_default())
                    .await
                    .unwrap_or_else(|e| {
                        eprintln!("list: {e}");
                        process::exit(1);
                    });
            }

            cli::TreeCommands::Add {
                name,
                description,
                edit,
            } => {
                forest::tree::add(name, description.unwrap_or_default(), edit)
                    .await
                    .unwrap_or_else(|e| {
                        eprintln!("tree add: {e}");
                        process::exit(1);
                    });
            }

            cli::TreeCommands::Remove { name } => {
                forest::tree::remove(&name).await.unwrap_or_else(|e| {
                    eprintln!("tree rm: {e}");
                    process::exit(1);
                });
            }

            cli::TreeCommands::Rename { name, new_name } => {
                forest::tree::rename(&name, new_name)
                    .await
                    .unwrap_or_else(|e| {
                        eprintln!("tree rename: {e}");
                        process::exit(1);
                    });
            }

            cli::TreeCommands::Show { name } => {
                forest::tree::show(&name).await.unwrap_or_else(|e| {
                    eprintln!("tree show: {e}");
                    process::exit(1);
                });
            }

            cli::TreeCommands::Edit { name } => {
                forest::tree::edit(&name).await.unwrap_or_else(|e| {
                    eprintln!("tree edit: {e}");
                    process::exit(1);
                });
            }
        },

        cli::Commands::Note { command } => match command {
            cli::NoteCommands::List { show_time_tracking } => {
                forest::notetaking::list(show_time_tracking)
                    .await
                    .unwrap_or_else(|e| {
                        eprintln!("note list: {e}");
                        process::exit(1);
                    });
            }

            cli::NoteCommands::Add { tree_name } => {
                forest::notetaking::add(tree_name, false)
                    .await
                    .unwrap_or_else(|e| {
                        eprintln!("note add: {e}");
                        process::exit(1);
                    });
            }
            cli::NoteCommands::Remove { uid } => {
                forest::notetaking::remove(&uid).await.unwrap_or_else(|e| {
                    eprintln!("note remove: {e}");
                    process::exit(1);
                });
            }
            cli::NoteCommands::Show { uid } => {
                forest::notetaking::show(&uid).await.unwrap_or_else(|e| {
                    eprintln!("note show: {e}");
                    process::exit(1);
                });
            }

            cli::NoteCommands::Edit { uid } => {
                forest::notetaking::edit(&uid).await.unwrap_or_else(|e| {
                    eprintln!("note edit: {e}");
                    process::exit(1);
                });
            }
        },

        cli::Commands::Switch { name } => {
            forest::tree::switch(&name).await.unwrap_or_else(|e| {
                eprintln!("switch: {e}");
                process::exit(1);
            });
        }

        cli::Commands::Start { tree_name, at } => {
            forest::timetracking::start(tree_name, at)
                .await
                .unwrap_or_else(|e| {
                    eprintln!("start: {e}");
                    process::exit(1);
                });
        }

        cli::Commands::Stop { at, no_note } => {
            // Note creation is enabled by default
            let create_note = !no_note;
            forest::timetracking::stop(at, create_note)
                .await
                .unwrap_or_else(|e| {
                    eprintln!("stop: {e}");
                    process::exit(1);
                });
        }
        cli::Commands::Status => {
            forest::timetracking::status().await.unwrap_or_else(|e| {
                eprintln!("status: {e}");
                process::exit(1);
            });
        }

        cli::Commands::Report { from, to, day } => {
            if day {
                forest::timetracking::report_day().await;
            } else {
                forest::timetracking::report(from, to).await;
            }
        }
    }
}
