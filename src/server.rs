use tokio::net::{TcpListener,TcpStream,UdpSocket};
use tokio::io::{AsyncReadExt,AsyncWriteExt};
use utils::generate_random_bytes;
use std::error::Error;
use std::sync::mpsc;
use std::str::from_utf8;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
struct Request {
    buffer_size: usize,
    reverse: bool
}

impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

async fn handle_connection(tx:mpsc::Sender<bool>,mut socket: TcpStream,port:i32)-> Result<(),Box<dyn Error>>{
    let (mut read_stream, mut write_stream) = socket.split();
    let mut data:Vec<u8> = vec![0;1024];
    let n = read_stream.read(&mut data).await?;
    if n == 0 {
        tx.send(true).unwrap();
        println!("Send done signal to TcpListener");
        return Ok(());
    }
    let serialiaze: &str = from_utf8(&data[..n]).unwrap();
    let deserialized: Request = serde_json::from_str(serialiaze).expect("Not able to parse the data");
    let address = format!("{}:{}","0.0.0.0",port);
    let sock = UdpSocket::bind(address).await?;
    println!("Started UDP servers...");
    write_stream.write_all(b"200").await?;
    println!("Waiting to receive message from client UDP socket");
    let (l,peer) = sock.recv_from(&mut data).await?;
    println!("recevied data from client");
    println!("Peer {:?}",peer);
    sock.send_to(b"..h.",peer).await?;
    println!("buffer size set to {:?}",deserialized.buffer_size);
    if !deserialized.reverse {
        let mut buf=  vec![0;deserialized.buffer_size];
        let size = read_stream.read(&mut data).await?;
        let msg = String::from_utf8_lossy(&data[..size]);
        println!("client message {:?}",msg);
        if msg == "Receive"{
            write_stream.write_all(b"Ok").await?;
            let (len, addr) = sock.recv_from(&mut buf).await?;
            println!("{:?} bytes received from {:?}",len,addr);
            write_stream.write_all(&format!("server received:{} bytes",len).into_bytes()).await?;
        }
        tx.send(true).unwrap();
    } else {
        let size =  read_stream.read(&mut data).await?;
        println!("n: {:?}, l:{:?}, size: {:?}",n,l,size);
        let msg  = String::from_utf8_lossy(&mut data[..size]);
        println!("client message {:?} ",msg);
        if msg == "Send"{
            
            write_stream.write_all(b"Ok").await?;
            let len = sock.send_to(&generate_random_bytes(deserialized.buffer_size),peer).await?;
            println!("{:?} bytes sent",len);
        }
        tx.send(true).unwrap();
    }
    Ok(())
} 

async fn run(port:i32) -> Result<(),Box<dyn Error>>{
	let addr = format!("0.0.0.0:{}",port);
	println!("Listening on address: {}",addr);
	let listener = TcpListener::bind(addr).await?;
	loop{
		let (socket,client_addr) = listener.accept().await?;
		println!("Received connection from {:?}",client_addr);
        let (tx, rx) = mpsc::channel();
		tokio::spawn(async move {
			_	= handle_connection(tx,socket,port).await;
		});

        let _ = rx.recv().unwrap();
        println!("TCP stream finished");

	}
}


pub async fn server(_buffer_size:usize,port:i32,) -> Result<(),Box<dyn Error>>{
    run(port).await?;
	Ok(())	
}
