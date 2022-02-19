use crate::db::FollowersDatabase;
use crate::models::followers::FollowersList;
use crate::network::ApiClient;
use crate::printer::{
    print_followers_unfollowers, print_following_metric, print_following_unfollowing,
    print_follwers_metric, print_heading, print_ok,
};
use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use std::error::Error;

pub fn check_handler(
    followers_db: &FollowersDatabase,
    network_api: &ApiClient,
) -> Result<(), Box<dyn Error>> {
    is_first(followers_db, network_api)?;
    let is_fetch_required = followers_db.is_refresh_required()?;
    if is_fetch_required {
        let username = followers_db.get_username()?;
        let followers =
            network_api.get_user_followers(generate_followers_url(username.to_string()))?;
        let following = network_api.get_user_followers(generate_following_url(username))?;
        let offline_db = followers_db.get_followers()?;
        let offline_following_db = followers_db.get_following()?;
        let unfollowers: FollowersList = offline_db
            .iter()
            .filter(|item| !followers.contains(item))
            .cloned()
            .collect();
        let new_followers: FollowersList = followers
            .iter()
            .filter(|item| !offline_db.contains(item))
            .cloned()
            .collect();
        let unfollowing: FollowersList = offline_following_db
            .iter()
            .filter(|item| !following.contains(item))
            .cloned()
            .collect();
        let new_following: FollowersList = following
            .iter()
            .filter(|item| !offline_following_db.contains(item))
            .cloned()
            .collect();
        print_heading("Followers".to_string());
        print_followers_unfollowers(new_followers, unfollowers);
        print_follwers_metric(format!("{}", &followers.len()));
        print_heading("Following".to_string());
        print_following_unfollowing(new_following, unfollowing);
        print_following_metric(format!("{}", &following.len()));
        followers_db.insert_followers(followers)?;
        followers_db.insert_following(following)?;
    } else {
        let offline_db = followers_db.get_followers()?;
        let offline_following_db = followers_db.get_following()?;
        let unfollowing: FollowersList = Vec::new();
        let following_list: FollowersList = Vec::new();
        let followerlist: FollowersList = Vec::new();
        let unfollowerlist: FollowersList = Vec::new();
        print_ok("You db was updated just few seconds ago".to_string());
        print_heading("Followers".to_string());
        print_followers_unfollowers(followerlist, unfollowerlist);
        print_follwers_metric(format!("{}", offline_db.len()));
        print_heading("Following".to_string());
        print_following_unfollowing(following_list, unfollowing);
        print_following_metric(format!("{}", &offline_following_db.len()));
    }
    Ok(())
}

pub fn clear_handler(followers_db: &FollowersDatabase) -> Result<(), Box<dyn Error>> {
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to clear database?")
        .interact()?
    {
        followers_db.nuke_db()?;
        print_ok("Succesfully nuked the database!".to_string());
    } else {
        print_ok("Okay! Not touching the database!".to_string());
    }
    Ok(())
}

pub fn status_handler(
    followers_db: &FollowersDatabase,
    network_api: &ApiClient,
) -> Result<(), Box<dyn Error>> {
    is_first(followers_db, network_api)?;
    let last_updated = followers_db.get_last_update_time()?;
    let username = followers_db.get_username()?;
    print_ok(format!(
        "The current GitHub profile that is attached is: {}",
        username
    ));
    print_ok(format!("Database was last updated at: {}", last_updated));
    Ok(())
}

fn is_first(
    followers_db: &FollowersDatabase,
    network_api: &ApiClient,
) -> Result<(), Box<dyn Error>> {
    let data = followers_db.get_is_first();
    match data {
        Ok(_) => Ok(()),
        Err(_) => {
            followers_db.nuke_db()?;
            let username: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Please enter your username")
                .allow_empty(false)
                .interact_text()?;
            let data = network_api.get_user_data(username.to_string())?;
            let followers =
                network_api.get_user_followers(generate_followers_url(data.login.to_string()))?;
            let following = network_api.get_user_followers(generate_following_url(data.login))?;
            followers_db.insert_followers(followers)?;
            followers_db.insert_username(username)?;
            followers_db.insert_following(following)?;
            followers_db.set_is_first("gunned and loaded".to_string())?;
            Ok(())
        }
    }
}

fn generate_following_url(username: String) -> String {
    format!(
        "https://api.github.com/users/{username}/following",
        username = username
    )
}

fn generate_followers_url(username: String) -> String {
    format!(
        "https://api.github.com/users/{username}/followers",
        username = username
    )
}

pub fn stats_handler(followers_db: &FollowersDatabase) -> Result<(), Box<dyn Error>> {
    let data = followers_db.get_is_first();
    match data {
        Ok(_) => {
            let followers = followers_db.get_following()?;
            let following = followers_db.get_followers()?;
            println!("{}_{}", &followers.len(), &following.len());
            Ok(())
        }
        Err(_) => {
            println!("err");
            Ok(())
        }
    }
}

pub fn shell_handler() -> Result<(), Box<dyn Error>> {
    let script = String::from(
        "function prompt_gunfollower() {
    result=$(gunfollower stats)
    if [[ \"$result\" == \"err\" ]]
    then
        p10k segment -f yellow -t \"logged out of gunfollower\"
    else
        my_arr=($(echo $result | sed 's/_/\n/g'))
        p10k segment -i '' -b green -f black -t \"followers: ${my_arr[2]} / following: ${my_arr[1]}\"
    fi
  }
",
    );
    println!(
            "{}\n\n        {}\n        {}\n        {}\n\n{}\n\n{}",
            style("Steps to add gunfollower p10k battery:").bold().underlined().green(),
            style("* Open .p10k.zsh file").italic().cyan(),
            style("* Copy the below function and paste it").italic().cyan(),
            style("* Add gunfollower to either POWERLEVEL9K_RIGHT_PROMPT_ELEMENTS or POWERLEVEL9K_LEFT_PROMPT_ELEMENTS").italic().cyan(),
            style(&script),
            style("If you are facing any issues, feel free to open an issue at https://github.com/DarthBenro008/gunfollower/issues").italic()
        );
    Ok(())
}
