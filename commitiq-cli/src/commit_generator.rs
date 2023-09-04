use crate::config_manager::read_api_key_from_file;
use crate::utils::get_gpt3_response;
use std::process::Command;
use std::str;
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

    let truncated_diff = if git_diff.len() > 4000 {
        &git_diff[0..4000]
    } else {
        git_diff
    };

    let prompt = format!(
        "Based on the following git diff, please generate a concise and descriptive commit message that summarizes the changes:\n\n{}\n\nOnly commit message is needed.",
        truncated_diff
    );
    let commit_message = get_gpt3_response(&api_key, &prompt).await?;

    Ok(commit_message.trim_matches('"').to_string())
}

// pub fn get_modified_files() -> Vec<String> {
//     let output = Command::new("git")
//         .args(&["status", "--porcelain"])
//         .output()
//         .expect("Failed to execute git command");

//     let output_str = str::from_utf8(&output.stdout).unwrap();
//     let mut modified_files = Vec::new();

//     let allowed_extensions = [
//         ".rs", ".js", ".json", ".cpp", ".cs", ".java", ".ts", ".go", ".py", ".txt", ".md", ".html",
//         ".css", ".scss", ".sass", ".less", ".xml", ".yaml", ".yml", ".toml", ".sh", ".bat", ".ps1",
//         ".php", ".rb", ".swift", ".h", ".m", ".mm", ".c", ".h", ".hpp", ".hxx", ".hh", ".cc",
//         ".cxx", ".c++", ".cs", ".scala", ".groovy", ".kt", ".kts", ".clj", ".cljs", ".cljc",
//         ".edn", ".lua", ".sql", ".pl", ".pm", ".t", ".r", ".rmd", ".cshtml", ".vb", ".vbs",
//     ];

//     for line in output_str.lines() {
//         let parts: Vec<&str> = line.split_whitespace().collect();
//         if parts.len() >= 2 {
//             let file_path = parts[1].to_string();
//             if allowed_extensions
//                 .iter()
//                 .any(|&ext| file_path.ends_with(ext))
//             {
//                 modified_files.push(file_path);
//             }
//         }
//     }

//     modified_files
// }

// pub fn display_and_select_files(files: Vec<String>) -> Vec<String> {
//     let mut all_files: Vec<&str> = vec!["* All"];
//     let files_str: Vec<&str> = files.iter().map(AsRef::as_ref).collect();
//     all_files.extend(&files_str);

//     let selections = MultiSelect::with_theme(&ColorfulTheme::default())
//         .with_prompt("Select files to add:")
//         .items(&all_files)
//         .interact()
//         .unwrap();

//     let mut selected_files = Vec::new();

//     for selection in selections {
//         if selection == 0 {
//             return files;
//         } else {
//             selected_files.push(files[selection - 1].clone());
//         }
//     }

//     selected_files
// }

pub async fn run_workflow(
    commit_message: &String,
    selected_files: &Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // for file in selected_files {
    //     println!("File: {}", file);
    // }
    // if selected_files.contains(&"* All".to_string()) {
    //     let _ = Command::new("git")
    //         .arg("add")
    //         .arg(".")
    //         .output()
    //         .expect("Failed to execute git add command");
    // } else {
    //     for file in selected_files {
    //         let _ = Command::new("git")
    //             .arg("add")
    //             .arg(file)
    //             .output()
    //             .expect("Failed to execute git add command for specific file");
    //     }
    // }

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
