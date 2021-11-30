use crate::models::followers::FollowersList;
use crate::models::user::User;
use reqwest::header;
use reqwest::header::ACCEPT;
use reqwest::header::USER_AGENT;

pub struct ApiClient {
    pub client: reqwest::blocking::Client,
}

impl ApiClient {
    pub fn new() -> ApiClient {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            USER_AGENT,
            header::HeaderValue::from_str("gunfollower").unwrap(),
        );
        headers.insert(
            ACCEPT,
            header::HeaderValue::from_str("application/vnd.github.v3+json").unwrap(),
        );
        let reqwest_client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build();
        ApiClient {
            client: reqwest_client.unwrap(),
        }
    }

    pub fn get_user_data(&self, username: String) -> Result<User, Box<dyn std::error::Error>> {
        let resp = self
            .client
            .get(format!(
                "https://api.github.com/users/{username}",
                username = username,
            ))
            .send()?;
        if resp.status() != 200 {
            return Err("Please enter a valid GitHub username".into());
        }
        let data = resp.json::<User>()?;
        Ok(data)
    }

    pub fn get_user_followers(
        &self,
        url: String,
    ) -> Result<FollowersList, Box<dyn std::error::Error>> {
        let mut page = 1;
        let mut followerlist: FollowersList = Vec::new();
        loop {
            let resp = self
                .client
                .get(format!(
                    "{url}?per_page=100&page={page}",
                    url = url,
                    page = page,
                ))
                .send()?;
            if resp.status() != 200 {
                return Err("GitHub rate limiter hit, please try again after an hour!".into());
            }
            let data = resp.json::<FollowersList>()?;
            if data.len() == 0 {
                break;
            }
            followerlist.extend(data);
            page = page + 1;
        }
        Ok(followerlist)
    }
}
