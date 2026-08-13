
use std::{io::{self,BufReader}, net::TcpStream};

use config::Config;

use crate::connections;

pub fn handle_connection(stream: TcpStream, settings: &Config) -> Result<(), std::io::Error>{

    println!("Connection established from {}", stream.peer_addr().unwrap());

    let buf_read: BufReader<&TcpStream> = BufReader::new(&stream);

    let proxied_connection = proxy_connection(buf_read, &settings);

    match proxied_connection{
        Ok(()) => {println!("Connection from {} proxied successfully", &stream.peer_addr()?.to_string());
                        return Ok(())},
        Err(error) => {eprintln!("Error creating proxy connection");
                            stream.shutdown(std::net::Shutdown::Both).unwrap();
                            return Err(io::Error::new(io::ErrorKind::ConnectionAborted, error))}
    };


}

fn proxy_connection(_read_buffer:BufReader<&TcpStream>, settings: &Config) -> Result<(), std::io::Error>{

    //TODO: Need to add check if remote_addr is a single value or an array, handle accordingly
    let remotes: Vec<config::Value> = settings.get_array("remote_addr").unwrap();


    //TODO: Need to add way to randomly select from one of the remote addrs given considering number of values, not just hardcode
    match remotes.get(0){
        Some(value) => {connections::TcpStream::connect(value.to_string()).unwrap(); Ok(())}
        None => {eprintln!("Error creating proxy connection");
                Err(io::Error::new(io::ErrorKind::ConnectionAborted, "error"))},
    }

}