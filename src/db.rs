use crate::models::followers::FollowersList;
use chrono::prelude::{DateTime, Utc};
use chrono::NaiveDateTime;
use std::io::{Error, ErrorKind};

pub struct FollowersDatabase {
    db: sled::Db,
}

impl FollowersDatabase {
    pub fn new() -> FollowersDatabase {
        match home::home_dir() {
            Some(mut path) => {
                path.push(".gunfollower");
                let db: sled::Db = sled::open(path).unwrap();
                FollowersDatabase { db }
            }
            None => {
                let db: sled::Db = sled::open("gunfollower_db").unwrap();
                FollowersDatabase { db }
            }
        }
    }

    pub fn insert_followers(&self, followers: FollowersList) -> Result<(), Error> {
        self.insert_last_update_time()?;
        let data = serde_json::to_string(&followers)?;
        self.db.insert("followers_data", data.as_bytes())?;
        Ok(())
    }

    pub fn get_followers(&self) -> Result<FollowersList, Error> {
        match self.db.get("followers_data")? {
            Some(bytes) => match String::from_utf8(bytes.to_vec()) {
                Ok(value) => {
                    let followers_list: FollowersList = serde_json::from_str(&value)?;
                    Ok(followers_list)
                }
                Err(_) => Err(Error::new(
                    ErrorKind::Other,
                    "Cannot fetch data from database",
                )),
            },
            None => Err(Error::new(ErrorKind::Other, "Cannot fetch from database")),
        }
    }

    pub fn insert_username(&self, username: String) -> Result<(), Error> {
        self.db.insert("username_data", username.as_bytes())?;
        Ok(())
    }

    pub fn get_username(&self) -> Result<String, Error> {
        match self.db.get("username_data")? {
            Some(bytes) => match String::from_utf8(bytes.to_vec()) {
                Ok(value) => Ok(value),
                Err(_) => Err(Error::new(ErrorKind::Other, "Error converting from array")),
            },
            None => Err(Error::new(ErrorKind::Other, "Cannot fetch from database")),
        }
    }
    pub fn get_is_first(&self) -> Result<String, Error> {
        match self.db.get("isFirst")? {
            Some(bytes) => Ok(String::from_utf8(bytes.to_vec()).unwrap()),
            None => Err(Error::new(ErrorKind::Other, "Cannot fetch from database")),
        }
    }

    pub fn set_is_first(&self, value: String) -> Result<(), Error> {
        self.db.insert("isFirst", value.as_bytes())?;
        Ok(())
    }

    pub fn insert_last_update_time(&self) -> Result<(), Error> {
        let utc: DateTime<Utc> = Utc::now();
        let bytes = &utc.timestamp().to_string();
        self.db.insert("token_time", bytes.as_bytes())?;
        Ok(())
    }

    pub fn get_last_update_time(&self) -> Result<String, Error> {
        match self.db.get("token_time")? {
            Some(bytes) => {
                let timestring = String::from_utf8(bytes.to_vec()).unwrap();
                let timestamp = timestring.parse::<i64>().unwrap();
                let time = NaiveDateTime::from_timestamp(timestamp, 0);
                let newdate = format!("{} {}", time.date(), time.time());
                Ok(newdate)
            }
            None => Err(Error::new(ErrorKind::Other, "Cannot de-serialize time")),
        }
    }

    pub fn is_refresh_required(&self) -> Result<bool, Error> {
        match self.db.get("token_time")? {
            Some(bytes) => {
                let timestring = String::from_utf8(bytes.to_vec()).unwrap();
                let timestamp = timestring.parse::<i64>().unwrap();
                let utc: DateTime<Utc> = Utc::now();
                let current_time = utc.timestamp();
                if (current_time - timestamp) > 1800 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            None => Err(Error::new(ErrorKind::Other, "Cannot de-serialize time")),
        }
    }

    pub fn nuke_db(&self) -> Result<(), Error> {
        self.db.clear()?;
        Ok(())
    }
}
