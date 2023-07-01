use tokio::net::UdpSocket;
use std::error::Error;
use clap::{Arg,Command};


#[derive(Debug,PartialEq)]
enum Mode {
    Server,
    Client
}

fn generate_random_bytes(size:usize) -> Vec<u8> {
        let random_bytes: Vec<u8> = (0..size).map(|_| { rand::random::<u8>() }).collect();
        random_bytes
}

async fn server(buffer_size:usize,port:i32,) -> Result<(),Box<dyn Error>>{
    let address = format!("{}:{}","0.0.0.0",port);
    let sock = UdpSocket::bind(address).await?;
    let mut buf=  vec![0;buffer_size];
    loop{
        let (len, addr) = sock.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?}",len,addr);
    }
}

async fn client(buffer_size:usize,remote_addr:String) -> Result<(),Box<dyn Error>> {
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

#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>>{
    let matches = Command::new("UDPCAT")
        .version("0.0.1")
        .author("Dinesh Arunachalam")
        .about("A tool to test the UDP fragmentation using various length")
        .arg(Arg::new("buffer_size")
             .short('l')
             .long("length")
             .help("buffer size to read or write"))
        .arg(Arg::new("mode")
             .short('m')
             .long("mode")
             .help("What mode to run the program in")
             .value_parser(["server","client"]))
        .arg(Arg::new("client")
             .short('c')
             .long("client")
             .help("run in client mode"))
        .arg(Arg::new("port")
             .short('p')
             .long("port")
             .default_value("50001")
             .value_parser(clap::value_parser!(i32))
             .help("run the server in specified port"))
        .arg(Arg::new("remote address")
             .short('r')
             .long("remote_address")
             .help("server address"))
        .get_matches();
    let buffer_str = matches.get_one::<String>("buffer_size");
    let buffer_size = match buffer_str{
        None => 1024,
        Some(s)  => {
            match s.parse::<usize>() {
                Ok(n) => n,
                Err(_) => panic!("Please provide a valid length {}",s),
            }
        }
    };
    let mode = match matches
        .get_one::<String>("mode")
        .expect("'MODE' is required and parsing will fail if its missing")
        .as_str()
        {
            "server" => Mode::Server,
            "client" => Mode::Client,
            _ => unreachable!(),
        };
    let buffer_size:usize =  buffer_size;
    let port = matches.get_one::<i32>("port").expect("could not parse the  port");

    if mode == Mode::Server{
        server(buffer_size, *port).await?
    }

    if mode == Mode::Client {
        let remote_address = matches.get_one::<String>("remote address").expect("could not parse the remote addresss");
        client(buffer_size,remote_address.to_string()).await?
    }
    Ok(())
}

