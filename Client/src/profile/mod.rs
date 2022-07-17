mod config;
//using nested path 
use std::{fmt,result}; 
/**

    A simple profile structure to hold all information
    regarding a user profile in the social network.

*/
pub struct Profile {
    //since we are implementing getters we can leave this to private 
    uid: u32,
    username: String,
    password: String
}

impl Profile {

    pub fn new(_username: &str, _password: &str) -> Profile {
        return Profile {
            uid: config::get_next_uid(),
            username: String::from(_username),
            password: String::from(_password)
        };
    }

    pub fn get_username(&self) -> &str {
        return &self.username;
    }

    pub fn get_password(&self) -> &str {
        return &self.password;
    }

    pub fn get_uid(&self) -> u32 {
        return self.uid;
    }
    
    pub fn equals(&self, other: &Profile) -> bool {
        return self.uid == other.get_uid();
    }
}

/*
to string method for profile needs to implement trait Display 
*/
impl fmt::Display for Profile{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> result::Result<(),fmt::Error>{
        // use write! macro just like println! macro, but output gets writen to
        // the formatter struct.
        write!(fmt,"{}-> uid, {} -> username,{} -> password",self.uid,self.username,self.password)
    }
}
