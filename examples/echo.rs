// A "hello world" echo server wit Tokio
/*
    This server will create a TCP listener, accept connections in a loop and write back 
    every thing that's read off each TCP connection

    Because the Tokio runtime uses a thread poll, each tcp connection is processed concurrently with all
    other TCP connection accross multiple threads.


    To see the server action, need to run this in one terminal
        cargo run --example echo

    and in another terminal you can run:
        cargo run --example connect 127.0.0.1:8080

    Each line you type in the   `connect` terminal should be echo'd back to you
    . if you open up multiple terminals running the `connect` example you should
    be able to see them all make progress simultaneously.

*/

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>>{
    // Allow passing an address to listen as the first argument of this program
    // Allow passing an address to listen as the first argument of this program
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());

    // Next up create a TCP listener which will listen for incomming connections

    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    loop {
        // Asynchronously wait for an inboud socket
        let ( mut socket, _) = listener.accept().await?;
        // And this where much of the magic of this server happens. we 
        // crucially want all clients to make progress concurrently, rather than
        // blocking one on completion of another.
        // To achive this we use the `tokio::spawn` function to execute the work in the background
        //
        //Essentially here we're executing anew task to run concurrently.
        // which will allow all of our clients to be processed concurrently.

        tokio::spawn(async move {
            let mut buffer = vec![0;1024];
            // In a loop, read data from the socket and write the data back
            let n = socket
                .read(&mut buffer)
                .await
                .expect("failed to read data from socket");
            if n == 0 {
                return ;
            }
            socket.write_all(&buffer[0..n])
                .await
                .expect("failed to write data to socket");
        });
    }
}

 
