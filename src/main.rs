use std::error::Error;
use clap::{Arg,Command,ArgAction};
mod server;
mod client;

#[derive(PartialEq)]
enum Mode {
    Server,
    Client
}


#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>>{
    let cmd = Command::new("udpcat")
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
             .short('h')
             .long("host_address")
             .help("server address which you want to connect"))
        .arg(Arg::new("reverse")
             .short('R')
             .long("reverse")
             .required(false)
             .help("run in reverse mode (server sends, client receives)")
             .action(ArgAction::SetTrue))
        .get_matches();
    let buffer_str = cmd.get_one::<String>("buffer_size");
    let buffer_size = match buffer_str{
        None => 1024,
        Some(s)  => {
            match s.parse::<usize>() {
                Ok(n) => n,
                Err(_) => panic!("Please provide a valid length {}",s),
            }
        }
    };
    let mode = match cmd
        .get_one::<String>("mode")
        .expect("'MODE' is required and parsing will fail if its missing")
        .as_str()
        {
            "server" => Mode::Server,
            "client" => Mode::Client,
            _ => unreachable!(),
        };
    let buffer_size:usize =  buffer_size;
    let port = cmd.get_one::<i32>("port").expect("could not parse the  port");

    if mode == Mode::Server{
        server::server(buffer_size, *port).await?
    }
    if mode == Mode::Client {
        let remote_address = cmd.get_one::<String>("remote address").expect("could not parse the remote addresss");
        let reverse:bool = cmd.get_flag("reverse");
        println!("{:?}",reverse);
        client::client(buffer_size,remote_address.to_string(),reverse).await?
    }
    Ok(())
}

