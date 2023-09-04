use crate::commit_generator;
use crate::config_manager;

use std::env;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use std::vec;

use colored::*;
use std::io::{self, Write};

pub enum Command {
    Config { api_key: String },
    GenerateCommit,
    Version,
    Unknown,
}

pub fn get_cli_matches() -> Command {
    let args: Vec<String> = env::args().collect();

    let program_name: &str = Path::new(&args[0]).file_name().unwrap().to_str().unwrap();

    if args.len() == 1 && program_name == "ciq" {
        return Command::GenerateCommit;
    }

    if args.len() == 2 {
        match args[1].as_str() {
            "--version" | "-v" => return Command::Version,
            _ => {}
        }
    }

    if args.len() >= 4 && args[1] == "config" && args[2] == "set" {
        return Command::Config {
            api_key: args[3].clone(),
        };
    }

    Command::Unknown
}

pub async fn handle_matches(command: Command) {
    match command {
        Command::Config { api_key } => {
            let home_dir = dirs::home_dir().expect("Could not determine home directory");
            let path = home_dir.join(".commitiq");
            config_manager::set_api_key(&api_key, path);
            println!("API key set to: {}", api_key);
        }
        Command::GenerateCommit => loop {
            // let modified_files = commit_generator::get_modified_files();
            // let selected_files = commit_generator::display_and_select_files(modified_files.clone());

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
                    commit_generator::run_workflow(&commit_message, &vec![])
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
                    println!("Run: git push");
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
        Command::Version => {
            println!("ciq v{}", env!("CARGO_PKG_VERSION"));
        }
        Command::Unknown => {
            println!("Unknown command");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cli_matches_generate_commit() {
        let args = vec!["commify".to_string()];
        std::env::set_var("ARGS_FOR_TEST", args.join(" "));
        let command = get_cli_matches();
        match command {
            Command::GenerateCommit => (),
            _ => panic!("Expected GenerateCommit"),
        }
        std::env::remove_var("ARGS_FOR_TEST");
    }

    #[test]
    fn test_get_cli_matches_config() {
        let args = vec![
            "commify".to_string(),
            "config".to_string(),
            "set".to_string(),
            "some_api_key".to_string(),
        ];
        std::env::set_var("ARGS_FOR_TEST", args.join(" "));
        let command = get_cli_matches();
        match command {
            Command::Config { api_key } => assert_eq!(api_key, "some_api_key"),
            _ => panic!("Expected Config"),
        }
        std::env::remove_var("ARGS_FOR_TEST");
    }

    #[test]
    fn test_get_cli_matches_unknown() {
        let args = vec!["commify".to_string(), "unknown_command".to_string()];
        std::env::set_var("ARGS_FOR_TEST", args.join(" "));
        let command = get_cli_matches();
        match command {
            Command::Unknown => (),
            _ => panic!("Expected Unknown"),
        }
        std::env::remove_var("ARGS_FOR_TEST");
    }
}
