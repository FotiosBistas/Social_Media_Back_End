
use crate::profile::Profile; 
use std::io;

///
/// Client calls method from this module to interact with the server
pub mod operations{

    use super::*; 


    pub fn signup(prof: &Profile){

    }

    pub fn login(prof: &Profile){
        println!("Trying to log in")
    }

    ///This function accepts the client who made the request. 
    /// 
    /// It will read the profile of an other client that it follows but not
    /// its directory. 
    pub fn access_profile(prof: &Profile) -> (){

    }

    ///This method adds a file to the clients local directory. 
    /// It should call some method to update the servers directory. 
    /// Should handle IO error properly. 
    pub fn add_file(prof: &Profile) -> Result<(),io::Error> {
        Ok(()) 
    }


} 