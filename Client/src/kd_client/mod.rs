
use crate::profile::Profile; 
use std::io;

///Request encapsulates who sends the request and the request type.
struct Request<'a>(&'a Profile, RequestType);
///Request type specifies what type of request is sent to the server.
pub enum RequestType{
    Login,
    SignUp,
}
///
/// Client calls method from this module to interact with the server
pub mod operations{
    use std::io::{Read, Sink, Write};
    use std::net::TcpStream;
    use crate::kd_client::RequestType::{Login, SignUp};
    use super::*;

    mod helper_methods {
        use super::*;

        fn create_request(request: &str,prof: &Profile) -> &[u8] {
            let prof = prof;
            let uid = &prof.get_uid().to_string();
            let username = prof.get_username();
            let password = prof.get_password();

            let mut buffer = String::with_capacity(request.len() + uid.len() + username.len() + password.len() + 4);

            buffer.insert_str(login.len(), request);
            buffer.insert(login.len(), ' ');
            buffer.insert_str(login.len(), uid);
            buffer.insert(login.len(), ' ');
            buffer.insert_str(login.len(), username);
            buffer.insert(login.len(), ' ');
            buffer.insert_str(login.len(), password);
            buffer.insert(login.len(), '\n');

            let buffer = buffer.as_bytes();
            &buffer
        }

        pub(crate) fn send_request(mut stream: TcpStream, request: Request) -> Result<(), &'static str> {
            let request_type = request.1;
            match request_type {
                Login => {
                    match stream.write(create_request("login",request.0)) {
                        Ok(_) => {}
                        _ => return Err("Error trying to write login request to TCP stream"),
                    }
                }
                SignUp => {
                    match stream.write(create_request("signup",request.0)) {
                        Ok(_) => {}
                        _ => return Err("Error trying to write signup request to TCP stream"),
                    }
                }
            }
            Ok(())
        }
    }

    pub fn signup(prof: &Profile) -> Result<(),&'static str>{

        println!("Trying to sign up");


        let stream = match TcpStream::connect("127.0.0.1:7878"){
            Ok(stream) => stream,
            _ => return Err("Could not connect to tcp stream")
        };

        match  helper_methods::send_request(stream,Request(prof, SignUp)){
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

        match helper_methods::send_request(stream,Request(prof,Login)){
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