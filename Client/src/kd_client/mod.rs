
use crate::profile::Profile; 
use std::io;


enum RequestType{
    Login,
    SignUp,
}
///
/// Client calls method from this module to interact with the server
pub mod operations{
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use crate::kd_client::RequestType::Login;
    use super::*;

    pub fn send_request(prof:&Profile,request_type:RequestType) -> Result<(),&'static str>{

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

        match  send_request(prof,Login){
            Ok(_) => {}
            Err(e) => {
                return Err("")
            }
        };

        Ok(())
    }

    pub fn login(prof: &Profile) -> Result<(),&'static str>{

        println!("Trying to log in");

        let mut buffer = [0;1024];
        let mut stream = match TcpStream::connect("127.0.0.1:7878"){
            Ok(stream) => stream,
            _ => return Err("Could not connect to tcp stream")
        };





        let n = match stream.read(&mut buffer){
            Ok(n) => n,
            _ => return Err("Error trying to read from buffer"),
        };

        println!("Read {} bytes from server.",n);

        println!("{:?}",buffer);

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