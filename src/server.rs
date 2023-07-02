use tokio::net::{TcpListener,TcpStream,UdpSocket};
use tokio::io::{AsyncReadExt,AsyncWriteExt};
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

    loop {
        let mut data:Vec<u8> = vec![0;1024];
        let n = read_stream.read(&mut data).await?;
        if n == 0 {
            tx.send(true).unwrap();
            println!("Send done signal to TcpListener");
            break;
        }
        let serialiaze: &str = from_utf8(&data[..n]).unwrap();
        let deserialized: Request = serde_json::from_str(serialiaze).expect("Not able to parse the data");
        write_stream.write_all(b"done\n").await?;
        if !deserialized.reverse {
            let address = format!("{}:{}","0.0.0.0",port);
            let sock = UdpSocket::bind(address).await?;
            println!("Started UDP servers...");
            println!("buffer size set to {:?}",deserialized.buffer_size);
            let mut buf=  vec![0;deserialized.buffer_size];
            let (len, addr) = sock.recv_from(&mut buf).await?;
            println!("{:?} bytes received from {:?}",len,addr);
            write_stream.write_all(&format!("server received:{} bytes",len).into_bytes()).await?;
            tx.send(true).unwrap();
        } else {
            std::thread::sleep(std::time::Duration::from_secs(60));
            tx.send(true).unwrap();
            println!("Send done signal to TcpListener");
        }
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
