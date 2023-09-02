mod cli_handlers;
mod commit_generator;
mod config_manager;
mod utils;

use colored::Colorize;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let command = cli_handlers::get_cli_matches();

    match command {
        cli_handlers::Command::Config { api_key } => {
            let home_dir = dirs::home_dir().expect("Could not determine home directory");
            let path = home_dir.join(".commitiq");
            config_manager::set_api_key(&api_key, path);
            println!("API key set to: {}", api_key);
        }
        cli_handlers::Command::GenerateCommit => loop {
            let git_diff = commit_generator::get_git_diff();
            let commit_message = commit_generator::generate_commit_message(&git_diff)
                .await
                .unwrap();
            let generating_message = "Generating commit message...".yellow();
            println!("{}", generating_message);
            sleep(Duration::from_millis(1000));
            println!("");

            let generate_message = "Generated commit message:".underline().white();
            let commit_message_str = commit_message.green().bold();
            println!("{}\n{}", generate_message, commit_message_str);

            let options = "(yes/no/new)".underline();
            print!("Do you want to commit with this message? {} ", options);
            io::stdout().flush().unwrap();

            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).unwrap();
            let user_input = user_input.trim();

            println!("");

            match user_input {
                "yes" | "y" => {
                    commit_generator::run_workflow(&commit_message)
                        .await
                        .unwrap();
                    println!("Committing...");
                    sleep(Duration::from_millis(500));

                    println!("Runing: git add .");
                    sleep(Duration::from_millis(500));

                    println!("Running: git commit -m \"{}\"", commit_message);
                    sleep(Duration::from_millis(500));
                    println!("");

                    let success_message = "Successfully committed:".white().underline();
                    println!("{}", success_message);
                    let commit_message_str = commit_message.green().bold();
                    println!("{}", commit_message_str);

                    println!("");
                    println!("You can now push your changes to the remote repository🚀.");
                    break;
                }
                "no" | "n" => {
                    let abort_message = "Commit aborted".red().bold();
                    println!("{}", abort_message);
                    break;
                }
                "new" => {
                    continue;
                }
                _ => {
                    println!("Invalid option. Please enter 'yes', 'no', or 'new'.");
                    println!("");
                }
            }
        },
        cli_handlers::Command::Version => {
            println!("ciq v{}", env!("CARGO_PKG_VERSION"));
        }
        cli_handlers::Command::Unknown => {
            println!("Unknown command");
        }
    }
}
