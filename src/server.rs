use tokio::net::UdpSocket;
use std::error::Error;

pub async fn server(buffer_size:usize,port:i32,) -> Result<(),Box<dyn Error>>{
    let address = format!("{}:{}","0.0.0.0",port);
    let sock = UdpSocket::bind(address).await?;
    let mut buf=  vec![0;buffer_size];
    loop{
        let (len, addr) = sock.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?}",len,addr);
    }
}
