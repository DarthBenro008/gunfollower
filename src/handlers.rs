use crate::db::FollowersDatabase;
use crate::network::ApiClient;
use std::error::Error;

pub fn check_handler(
    followers_db: &FollowersDatabase,
    network_api: &ApiClient,
) -> Result<(), Box<dyn Error>> {
    is_first(followers_db, network_api)?;
    let username = followers_db.get_username()?;
    let followers = network_api
        .get_user_followers(generate_followers_url(username))
        .unwrap();
    // Do compare and generate results
    Ok(())
}

pub fn clear_handler(followers_db: &FollowersDatabase) -> Result<(), Box<dyn Error>> {
    // confirm from user
    followers_db.nuke_db()?;
    // print confirmation
    Ok(())
}

pub fn status_handler(
    followers_db: &FollowersDatabase,
    network_api: &ApiClient,
) -> Result<(), Box<dyn Error>> {
    is_first(followers_db, network_api)?;
    let username = followers_db.get_username()?;
    println!("{}", username);
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
            // Ask for username
            let username = "DarthBenro008";
            let data = network_api.get_user_data(username.to_string())?;
            println!("{}", generate_followers_url(data.login.to_string()));
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
