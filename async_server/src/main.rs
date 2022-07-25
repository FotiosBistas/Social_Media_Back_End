use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}};

#[tokio::main]
async fn main() {
    let listener = match TcpListener::bind("localhost:8080").await{
        Ok(listener) =>listener,
        Err(e) => panic!("Could not open tcp stream: {}",e),
    };

    let (mut socket,mut addr) = match listener.accept().await{
        Ok((socket,addr)) => (socket,addr),
        Err(e) => panic!("Could not accept connection: {}",e)
    }; 

    loop{
        let mut buffer = [0u8;1024]; 

        let bytes_read =  socket.read(&mut buffer).await.unwrap(); 

        socket.write_all(&buffer[..bytes_read]).await.unwrap(); 
    }
}