use crate::db::FollowersDatabase;
use crate::models::followers::FollowersList;
use crate::network::ApiClient;
use crate::printer::{print_followers_unfollowers, print_ok};
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
        let followers = network_api.get_user_followers(generate_followers_url(username))?;
        let offline_db = followers_db.get_followers()?;
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
        print_followers_unfollowers(new_followers, unfollowers);
        print_ok(format!("You currently have {} followers", &followers.len()));
        followers_db.insert_followers(followers)?;
    } else {
        let offline_db = followers_db.get_followers()?;
        let followerlist: FollowersList = Vec::new();
        let unfollowerlist: FollowersList = Vec::new();
        print_ok("You db was updated just few seconds ago".to_string());
        print_followers_unfollowers(followerlist, unfollowerlist);
        print_ok(format!("You currently have {} followers", offline_db.len()));
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
            let followers = network_api.get_user_followers(generate_followers_url(data.login))?;
            followers_db.insert_followers(followers)?;
            followers_db.insert_username(username.to_string())?;
            followers_db.set_is_first("gunned and loaded".to_string())?;
            Ok(())
        }
    }
}

fn generate_followers_url(username: String) -> String {
    format!(
        "https://api.github.com/users/{username}/followers",
        username = username
    )
}
