mod cli;
mod models;
mod network;

use cli::CommandLineArgs;
use network::ApiClient;
use structopt::StructOpt;

fn main() {
    let CommandLineArgs { cmd } = CommandLineArgs::from_args();
    match cmd {
        check => {
            let api = ApiClient::new();
            let data = api.get_user_data(String::from("DarthBenro008")).unwrap();
        }
    }
}
