
use crate::profile::Profile; 
use std::{io, ops};

///Request encapsulates who sends the request and the request type. (profile, request_type)
struct Request<'a>(&'a Profile, RequestType);
///Request type specifies what type of request is sent to the server.
pub enum RequestType{
    Login,
    SignUp,
}



///
/// Client calls methods from this module to interact with the server
pub mod operations{
    use std::error::Error;
    use std::fs;
    use std::fs::File;
    use std::io::{ErrorKind, Read, Sink, Write};
    use std::net::TcpStream;
    use crate::kd_client::RequestType::{Login, SignUp};
    use super::*;

    mod helper_methods {
        use super::*;


        pub(crate) fn send_request(mut stream: TcpStream, request: Request) -> Result<(), &'static str> {
            let request_type = request.1;
            let prof = request.0;
            let uid = &prof.get_uid().to_string();
            let username = prof.get_username();
            let password = prof.get_password();

            match request_type {
                Login => {
                    let req = "login";
                    let req = &format!("{} {} {} {}{}",req,uid,username,password,'\n');
                    match stream.write(req.as_bytes()) {
                        Ok(_) => {}
                        _ => return Err("Error trying to write login request to TCP stream"),
                    }
                }
                SignUp => {
                    let req = "signup";
                    let req = &format!("{} {} {} {}{}",req,uid,username,password,'\n');
                    match stream.write(req.as_bytes()) {
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


    ///Read file reads a specific file given as a name using the id of the client.
    ///Must accept the a profile instance and a file_type = Others or Profile
    ///Example: Profile_Xclient1 or Others_Xclient1
    fn read_local_file(prof: &Profile,file_type:&str) -> Result<&'static str, Box<dyn Error>> {

        let filename = &format!("{}{}",file_type,prof.get_uid());

        //if file doesn't exist create it else return the contents
        let mut contents = match fs::read_to_string(filename){
            Err(E) => match E.kind(){
                ErrorKind::NotFound => match File::create(filename) {
                    Ok(file) => file,
                    Err(e) => return Err(Box::new(e)),
                },
                other_error => return Err(Box::new(E)),
            },
            Ok(contents) => return Ok(&contents),
        };
        //if the function hasn't returned already it means it created the file
        let mut buffer = String::new();
        let contents = contents.read_to_string(&mut buffer);
        Ok(&buffer.clone())
    }

} 