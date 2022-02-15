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

pub fn print_following_metric(data: String) {
    println!(
        "✅ {} {} {}",
        style("You currently are following".to_string())
            .for_stdout()
            .green(),
        style(data).for_stdout().green().bold().underlined(),
        style("developers".to_string()).for_stdout().green()
    )
}

pub fn print_follwers_metric(data: String) {
    println!(
        "✅ {} {} {}",
        style("You currently have".to_string()).for_stdout().green(),
        style(data).for_stdout().green().bold().underlined(),
        style("followers!").for_stdout().green()
    )
}

pub fn print_heading(data: String) {
    println!("\n{}", style(data).cyan().underlined())
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

pub fn print_following_unfollowing(new_followers: FollowersList, unfollowers: FollowersList) {
    if new_followers.is_empty() && unfollowers.is_empty() {
        println!(
            "{}",
            style("😇 You did not follow or unfollow anyone!".to_string()).green()
        );
        return;
    }
    if !new_followers.is_empty() {
        println!(
            "{}",
            style("🤩 New folks whom you follow! :")
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
            style("😬 Folks whom you unfollowed: ")
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
