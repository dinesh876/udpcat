use tokio::net::{TcpListener,TcpStream};
use tokio::io::{AsyncReadExt,AsyncWriteExt};
use std::error::Error;
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

async fn handle_connection(mut socket: TcpStream) -> Result<(),Box<dyn Error>>{
	let (mut read_stream, mut write_stream) = socket.split();

	loop {
		let mut data:Vec<u8> = vec![0;1024];
		let n = read_stream.read(&mut data).await?;
		if n == 0 {
			break;
		}
        let serialiaze: &str = from_utf8(&data[..n]).unwrap();
        let deserialized: Request = serde_json::from_str(serialiaze).expect("Not able to parse the data");
        println!("Buffer Size: {:?} , Reverse: {:?}",deserialized.buffer_size,deserialized.reverse);
		write_stream.write_all(b"write done\n").await?;
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
		tokio::spawn(async move {
			_	= handle_connection(socket).await;
		});

	}
}


pub async fn server(_buffer_size:usize,port:i32,) -> Result<(),Box<dyn Error>>{
	run(port).await?;
	Ok(())	
		/*
		   let address = format!("{}:{}","0.0.0.0",port);
		   let sock = UdpSocket::bind(address).await?;
		   let mut buf=  vec![0;buffer_size];
		   loop{
		   let (len, addr) = sock.recv_from(&mut buf).await?;
		   println!("{:?} bytes received from {:?}",len,addr);
		   }
		   */
}
