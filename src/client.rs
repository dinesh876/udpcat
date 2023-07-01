use tokio::net::UdpSocket;
use std::error::Error;
use utils::generate_random_bytes;

pub async fn client(buffer_size:usize,remote_addr:String) -> Result<(),Box<dyn Error>> {
    let sock = UdpSocket::bind("0.0.0.0:0").await?;
    let remote_address: String = remote_addr;
    match sock.connect(&remote_address).await{
        Ok(addr) => println!("Connected to remote server {:?}",addr),
        Err(error) => panic!("Not able to connect to remote server {:?}",error) 
    };
    let len = sock.send(&generate_random_bytes(buffer_size)).await?;
    println!("{:?} bytes sent", len);
    Ok(drop(sock))
}
