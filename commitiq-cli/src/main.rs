mod cli_handlers;
mod commit_generator;
mod config_manager;
mod utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let command = cli_handlers::get_cli_matches();

    cli_handlers::handle_matches(command).await;
}
