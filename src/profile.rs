use crate::config;
/**

    A simple profile structure to hold all information
    regarding a user profile in the social network.

*/
pub struct Profile {
    pub uid: u32,
    pub username: String,
    pub password: String
}

impl Profile {

    pub fn new(_username: &str, _password: &str) -> Profile {
        return Profile {
            uid: config::get_next_uid(),
            username: String::from(_username),
            password: String::from(_password)
        };
    }

    pub fn to_string(&self) -> String {
        return format!("Profile [{}, {}, {}]", self.uid, self.username, self.password);
    }

    pub fn get_username(&self) -> String {
        return self.username.clone();
    }

    pub fn get_uid(&self) -> u32 {
        return self.uid;
    }
    
    pub fn equals(&self, other: Profile) -> bool {
        return self.uid == other.get_uid();
    }
}