use crate::models::followers::FollowersList;
use console::style;

pub fn print_error(action: &str, error: &std::boxed::Box<dyn std::error::Error>) {
    println!(
        "{} {}\n{}: {}",
        style("😭 Uh oh, somthing went wrong while")
            .for_stderr()
            .bold()
            .red(),
        style(action).for_stderr().bold().red(),
        style("Reason").for_stderr().red().underlined(),
        style(error).for_stderr().red()
    );
}

pub fn print_ok(data: String) {
    println!("✅ {}", style(data).for_stdout().green())
}

pub fn print_followers_unfollowers(new_followers: FollowersList, unfollowers: FollowersList) {
    if new_followers.is_empty() && unfollowers.is_empty() {
        println!(
            "{}",
            style("😎 Chill! No one unfollowed or followed you!".to_string()).green()
        );
        return;
    }
    if !new_followers.is_empty() {
        println!(
            "{}",
            style("🤝 New folks who followed you :")
                .for_stdout()
                .bold()
                .green()
        );
        for new_follower in &new_followers {
            println!(
                "  {} -> {}",
                style(new_follower.login.to_string()).green(),
                style(new_follower.html_url.to_string()).cyan()
            );
        }
    }
    if !unfollowers.is_empty() {
        println!(
            "{}",
            style("😫 Folks who unfollowed you: ")
                .for_stdout()
                .bold()
                .red()
        );
        for unfollower in &unfollowers {
            println!(
                "  {} -> {}",
                style(unfollower.login.to_string()).green(),
                style(unfollower.html_url.to_string()).cyan()
            );
        }
    }
}
