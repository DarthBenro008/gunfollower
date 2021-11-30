mod cli;
mod db;
mod handlers;
mod models;
mod network;

use cli::{CommandLineArgs, Commands};
use db::FollowersDatabase;
use handlers::*;
use network::ApiClient;
use structopt::StructOpt;

fn main() {
    let CommandLineArgs { cmd } = CommandLineArgs::from_args();
    let followers_db = FollowersDatabase::new();
    let api = ApiClient::new();
    match cmd {
        Commands::Check => match check_handler(&followers_db, &api) {
            Ok(_) => return,
            Err(err) => println!("error {}", err),
        },
        Commands::Clear => match clear_handler(&followers_db) {
            Ok(_) => return,
            Err(err) => println!("error {}", err),
        },
        Commands::Status => match status_handler(&followers_db, &api) {
            Ok(_) => return,
            Err(err) => println!("error {}", err),
        },
    }
}
