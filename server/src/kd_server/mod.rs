
use crate::server_struct::Server;

pub mod operations {
    use std::io::Read;
    use std::net::TcpStream;
    use super::*;

    pub fn search(server: &Server){

    }

    pub fn update_directories(server: &Server){

    }

    pub fn handle_connection(mut stream: TcpStream){
        println!("Connection started");
        let mut buffer = [0;10];
        let n = stream.read(& mut buffer[..]).unwrap();

        println!("Bytes read: {:?}", &buffer[..]);
    }
} 