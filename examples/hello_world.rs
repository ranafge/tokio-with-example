
use std::error::Error;

use tokio::{io::AsyncWriteExt, net::TcpStream};

/*
    A simple client that opens a TCP stream, writes "hello world\n", and closes the connection
    To start a server that this client can talk to on port 6142 using this command
    ncat -l 6142 // ncat is a networking utility tool that manipulate network connection over tcp and udp -l is incomming connection
    its client and server as uses.
*/

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // Open a TCP stream to the socket address
    //
    // Note that this is the Tokio TcpStrea, which is fully async.
    let mut stream = TcpStream::connect("127.0.0.1:6142").await?;
    println!("Created stream");

    let result = stream.write_all(b"hello world\n").await;
    println!("wrote to stream; success={:?}", result.is_ok());
    Ok(())
}