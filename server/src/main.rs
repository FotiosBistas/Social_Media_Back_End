mod thread_pool;
mod kd_server;
mod server_struct;

use std::{net::TcpListener, io::Error, thread};
use crate::thread_pool::ThreadPool;
use crate::kd_server::operations;


fn main(){


    let pool = ThreadPool::new(8);


    //handle error 
    let tcp_listener = match TcpListener::bind("127.0.0.1:178"){
        Ok(listener) => listener,
        Err(error) => {
            panic!("Could not bind tcp listener: {}",error);
        }
    };

    let tcp_listener = tcp_listener.incoming();

    for stream in tcp_listener{
        let stream = stream.expect("failed");

        pool.execute(|| {operations::handle_connection(stream)});

    }

}