use crate::config_manager::read_api_key_from_file;
use crate::utils::get_gpt3_response;
use std::process::Command;
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum GenerateCommitError {
    ReqwestError(reqwest::Error),
    HomeDirNotFound,
}

impl fmt::Display for GenerateCommitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GenerateCommitError::ReqwestError(err) => write!(f, "ReqwestError: {}", err),
            GenerateCommitError::HomeDirNotFound => write!(f, "Home directory not found"),
        }
    }
}

impl Error for GenerateCommitError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            GenerateCommitError::ReqwestError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for GenerateCommitError {
    fn from(err: reqwest::Error) -> GenerateCommitError {
        GenerateCommitError::ReqwestError(err)
    }
}

pub fn get_git_diff() -> String {
    let output = Command::new("git")
        .arg("diff")
        .output()
        .expect("Failed to execute git command");

    String::from_utf8(output.stdout).expect("Failed to convert to String")
}

pub async fn generate_commit_message(git_diff: &str) -> Result<String, GenerateCommitError> {
    if git_diff.trim().is_empty() {
        println!("No changes detected.");
        std::process::exit(0);
    }

    let mut api_key = String::new();

    let home_dir = dirs::home_dir().ok_or(GenerateCommitError::HomeDirNotFound)?;
    let path = home_dir.join(".commitiq");

    if let Ok(key) = read_api_key_from_file(path) {
        api_key = key;
    }

    let prompt = format!(
        "Based on the following git diff, please generate a concise and descriptive commit message that summarizes the changes:\n\n{}\n\nOnly commit message is needed.",
        git_diff
    );
    let commit_message = get_gpt3_response(&api_key, &prompt).await?;

    Ok(commit_message.trim_matches('"').to_string())
}

pub async fn run_workflow(commit_message: &String) -> Result<(), Box<dyn std::error::Error>> {
    let _ = Command::new("git")
        .arg("add")
        .arg(".")
        .output()
        .expect("Failed to execute git add command");

    let _ = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&commit_message)
        .output()
        .expect("Failed to execute git commit command");

    Ok(())
}
