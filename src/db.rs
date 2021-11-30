use crate::models::followers::FollowersList;
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

    pub fn nuke_db(&self) -> Result<(), Error> {
        self.db.clear()?;
        Ok(())
    }
}
