
use crate::server_struct::Server;

pub mod operations {
    use std::io;
    use std::str;
    use std::io::{Error, Read, Write};
    use std::net::TcpStream;
    use super::*;

    pub fn search(server: &Server){

    }

    pub fn update_directories(server: &Server){

    }

    pub fn handle_connection(mut stream: &TcpStream) -> Result<(),&'static str>{
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

        let request = buffer.next().unwrap();
        let uid = buffer.next().unwrap();
        let username = buffer.next().unwrap();
        let password = buffer.next().unwrap() ;

        let request = match str::from_utf8(request) {
            Ok(v) => v,
            Err(e) => return Err("Invalid UTF-8 sequence"),
        };

        let uid = match str::from_utf8(uid) {
            Ok(v) => v,
            Err(e) => return Err("Invalid UTF-8 sequence"),
        };

        let username = match str::from_utf8(username) {
            Ok(v) => v,
            Err(e) => return Err("Invalid UTF-8 sequence"),
        };

        let password = match str::from_utf8(password) {
            Ok(v) => v,
            Err(e) => return Err("Invalid UTF-8 sequence"),
        };


        println!("Welcome client {}",uid);



        Ok(())

    }
} 