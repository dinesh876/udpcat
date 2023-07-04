use tokio::net::{UdpSocket,TcpStream};
use tokio::io::{AsyncReadExt,AsyncWriteExt};
use std::error::Error;
use serde::Serialize;
use utils::generate_random_bytes;

#[derive(Serialize)]
struct Request{
    buffer_size: usize,
    reverse: bool
}


pub async fn client(buffer_size:usize,remote_addr:String,reverse: bool) -> Result<(),Box<dyn Error>> {
    let remote_ref = remote_addr.clone();
    let mut stream = TcpStream::connect(remote_addr).await?;

    let data =  Request {
        buffer_size,
        reverse
    };
    let json_data = serde_json::to_string(&data).unwrap();
    stream.write_all(json_data.as_bytes()).await?;
    let mut buffer = [0; 1024];
    let size = stream.read(&mut buffer).await?;
    let message = String::from_utf8_lossy(&buffer[..size]);
    println!("Server says: {:?}",message);
    if message == "200" {
        connect_udp_server(buffer_size,remote_ref,reverse,stream).await?;
    }
    Ok(())
}
async fn connect_udp_server(buffer_size:usize,remote_addr:String,reverse:bool,mut stream:TcpStream) -> Result<(),Box<dyn Error>>{

    let sock = UdpSocket::bind("0.0.0.0:0").await?;
    let remote_address: String = remote_addr;
    let mut data  = [0;1024];
    match sock.connect(&remote_address).await{
        Ok(addr) => println!("Connected to remote server {:?}",addr),
        Err(error) => panic!("Not able to connect to remote server {:?}",error) 
    };
    println!("sending message to udp server");
    sock.send(b"..:.").await?;
    println!("Sent");
    let _ = sock.recv_from(&mut data).await?;
    println!("Recevied reply from udp server");
    if !reverse{
        stream.write_all(b"Receive").await?;
        println!("Client sending...server receiving");
        let n = stream.read(&mut data).await?;
        let msg =  String::from_utf8_lossy(&data[..n]);
        if msg == "Ok"{
            let len = sock.send(&generate_random_bytes(buffer_size)).await?;
            println!("{:?} bytes sent to server", len);
        }
    }  else {
        stream.write_all(b"Send").await?;
        println!("Server sending...client receiving");
        let mut buf =  vec![0;buffer_size];
        let n = stream.read(&mut data).await?;
        let msg = String::from_utf8_lossy(&data[..n]);
        if msg == "Ok"{
            let (len,addr) = sock.recv_from(&mut buf).await?;
            println!("{:?} bytes received from {:?}",len,addr);
        }
    }
    Ok(drop(sock))
}
