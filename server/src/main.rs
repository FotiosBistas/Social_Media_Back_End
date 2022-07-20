mod thread_pool;
mod kd_server;
mod server_struct;

use std::{net::TcpListener, io::Error, thread};
use std::net::Shutdown;
use crate::thread_pool::ThreadPool;
use crate::kd_server::operations;


fn main(){


    let pool = ThreadPool::new(8);


    //handle error 
    let tcp_listener = match TcpListener::bind("127.0.0.1:7878"){
        Ok(listener) => listener,
        Err(error) => {
            panic!("Could not bind tcp listener: {}",error);
        }
    };

//
    for stream in tcp_listener.incoming(){
        match stream{
            Ok(stream) => {
                pool.execute(move || {
                    operations::handle_connection(&stream).unwrap_or_else(|err|{
                        eprintln!("Error handling connection: {}", err);
                        stream.shutdown(Shutdown::Both).expect("Couldn't shutdown stream");
                    });
                });
            }
            Err(e) => println!("Error {} occurred trying to establish connection",e),
        };
    }
    println!("Exiting main");
}