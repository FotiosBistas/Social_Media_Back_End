
use crate::profile::Profile; 
use std::io;


pub enum RequestType{
    Login,
    SignUp,
}
///
/// Client calls method from this module to interact with the server
pub mod operations{
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use crate::kd_client::RequestType::{Login, SignUp};
    use super::*;

    pub fn send_request(mut stream: TcpStream, prof:&Profile, request_type:RequestType) -> Result<(),&'static str>{

        let uid = &prof.get_uid().to_string();
        let username = prof.get_username() ;
        let password = prof.get_password();

        match request_type {
            Login => {
                let request = "login";
                let mut login = String::with_capacity(request.len() + uid.len() + username.len() + password.len() + 4);

                login.insert_str(login.len(), request);
                login.insert(login.len(),' ');
                login.insert_str(login.len(), uid);
                login.insert(login.len(),' ');
                login.insert_str(login.len(),username);
                login.insert(login.len(),' ');
                login.insert_str(login.len(),password);
                login.insert(login.len(),'\n');

                let login = login.as_bytes();
                match stream.write(login) {
                    Ok(_) => {}
                    _ => return Err("Error trying to write login request to TCP stream"),
                }
            }
            SignUp => {

            }
        }
        Ok(())
    }

    pub fn signup(prof: &Profile) -> Result<(),&'static str>{

        println!("Trying to sign up");


        let stream = match TcpStream::connect("127.0.0.1:7878"){
            Ok(stream) => stream,
            _ => return Err("Could not connect to tcp stream")
        };

        match  send_request(stream,prof,SignUp){
            Ok(_) => {}
            Err(e) => {
                return Err(e)
            }
        };

        Ok(())
    }

    pub fn login(prof: &Profile) -> Result<(),&'static str>{

        println!("Trying to log in");

        let stream = match TcpStream::connect("127.0.0.1:7878"){
            Ok(stream) => stream,
            _ => return Err("Could not connect to tcp stream")
        };

        match send_request(stream,prof,Login){
            Ok(_) => {}
            Err(e) => {
                return Err(e)
            }
        };


        Ok(())
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