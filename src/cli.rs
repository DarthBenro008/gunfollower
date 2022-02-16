use structopt::StructOpt;

#[derive(Debug, StructOpt, PartialEq)]
/// An CLI that helps you track who unfollowed you, written purely in Rust
///         
///        * Gun on those who unfollowed you!
///        * Has powerlevel10k integration and support
///
/// Developed by Hemanth Krishna (https://github.com/DarthBenro008)
#[structopt(
    name = "gunfollower",
    about = "A simple CLI to check who unfollowed you on github",
    verbatim_doc_comment
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub cmd: Option<Commands>,
}

#[derive(Debug, StructOpt, PartialEq)]
pub enum Commands {
    /// Checks who unfollowed you
    Check,
    /// Gives the status of the connected account,
    Status,
    /// Clears the connected account and the followers database
    Clear,
    /// Instructions on how to add p10K support
    Shell,
    /// Service for p10k stats
    Stats,
}
