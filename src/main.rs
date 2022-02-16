mod cli;
mod db;
mod handlers;
mod models;
mod network;
mod printer;

use cli::{CommandLineArgs, Commands};
use db::FollowersDatabase;
use handlers::*;
use network::ApiClient;
use printer::print_error;
use structopt::StructOpt;

fn main() {
    let CommandLineArgs { cmd } = CommandLineArgs::from_args();
    let followers_db = FollowersDatabase::new();
    let api = ApiClient::new();
    match cmd {
        Some(cmd) => match cmd {
            Commands::Check => match check_handler(&followers_db, &api) {
                Ok(_) => {}
                Err(err) => print_error("Failed to check who unfollowed you", &err),
            },
            Commands::Clear => match clear_handler(&followers_db) {
                Ok(_) => {}
                Err(err) => print_error("Failed to clear database", &err),
            },
            Commands::Status => match status_handler(&followers_db, &api) {
                Ok(_) => {}
                Err(err) => print_error("Failed to fetch status", &err),
            },
            Commands::Shell => match shell_handler() {
                Ok(_) => {}
                Err(err) => print_error("Something went wrong", &err),
            },
            Commands::Stats => match stats_handler(&followers_db) {
                Ok(_) => {}
                Err(err) => print_error("Something went wrong", &err),
            },
        },
        _ => match check_handler(&followers_db, &api) {
            Ok(_) => {}
            Err(err) => print_error("Failed to check who unfollowed you", &err),
        },
    }
}
