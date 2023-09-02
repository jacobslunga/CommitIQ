use std::env;
use std::path::Path;

pub enum Command {
    Config { api_key: String },
    GenerateCommit,
    Unknown,
}

pub fn get_cli_matches() -> Command {
    let args: Vec<String> = env::args().collect();

    let program_name: &str = Path::new(&args[0]).file_name().unwrap().to_str().unwrap();

    if args.len() == 1 && program_name == "ciq" {
        return Command::GenerateCommit;
    }

    if args.len() >= 4 && args[1] == "config" && args[2] == "set" {
        return Command::Config {
            api_key: args[3].clone(),
        };
    }

    Command::Unknown
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
