
use crate::profile::Profile; 
use std::io;

///
/// Client calls method from this module to interact with the server
pub mod operations{
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use super::*;


    pub fn signup(prof: &Profile) -> Result<(),&'static str>{
        Ok(())
    }

    pub fn login(prof: &Profile) -> Result<(),&'static str>{

        println!("Trying to log in");

        let mut buffer = [0;1024];
        let mut stream = match TcpStream::connect("127.0.0.1:7878"){
            Ok(stream) => stream,
            _ => return Err("Could not connect to tcp stream")
        };

        let mut login = b"log in";



        match stream.write(b"log in\n") {
            Ok(_) => {}
            _ => return Err("Error trying to write to TCP stream"),
        }

        let n = match stream.read(&mut buffer){
            Ok(n) => n,
            _ => return Err("Error trying to read from buffer"),
        };

        println!("Read {} bytes from server.",n);


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