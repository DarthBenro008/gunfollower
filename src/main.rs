mod cli;
mod models;
mod network;

use cli::{CommandLineArgs, Commands};
use network::ApiClient;
use structopt::StructOpt;

fn main() {
    let CommandLineArgs { cmd } = CommandLineArgs::from_args();
    match cmd {
        Commands::Check => {
            let api = ApiClient::new();
            let data = api.get_user_data(String::from("DarthBenro008")).unwrap();
            let followers = api.get_user_followers(data.followers_url).unwrap();
            println!("{:#?}", followers.len());
        }
        Commands::Clear => {}
        Commands::Status => {}
    }
}
