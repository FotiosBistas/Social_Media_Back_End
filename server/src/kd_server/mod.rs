
use crate::server_struct::Server;

pub mod operations {
    use std::io;
    use std::io::{Error, Read};
    use std::net::TcpStream;
    use super::*;

    pub fn search(server: &Server){

    }

    pub fn update_directories(server: &Server){

    }

    pub fn handle_connection(mut stream: TcpStream) -> Result<(),&'static str>{
        println!("Connection started");
        let mut buffer = [0;1024];
        let n = match stream.read(& mut buffer[..]){
            Ok(n) => n,
            _ => return Err("Error trying to read from buffer"),
        };

        println!("Read {} bytes from buffer.",n);

        //first part of the array not containing junk data
        let index = buffer.iter().position(|x| *x == 10).unwrap();
        let mut buffer = buffer.split_at(index);
        let mut buffer = buffer.0.split(|x|*x == 32);

        let request = buffer.next();
        let uid = buffer.next();
        let username = buffer.next();
        let password = buffer.next();

        println!("Request: {:?}, uid: {:?}, username: {:?}, password: {:?}",request,uid,username,password);

        Ok(())

    }
} 