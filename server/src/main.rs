mod thread_pool;
mod kd_server;
mod server_struct;

use std::{net::TcpListener, io::Error};

fn main(){

    //handle error 
    let tcp_listener = match TcpListener::bind("127.0.0.1:178"){
        Ok(listener) => Ok(listener),
        Err(error) => {
            panic!("Could not bind tcp listener: {}",error);
        }
    };

    let tcp_listener = tcp_listener.unwrap().incoming();

    for stream in tcp_listener{
        stream.expect("failed");


    }

}