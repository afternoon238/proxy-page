use std::{
    io::{self},
    net::TcpStream,
    thread,
};

use config::Config;

pub fn handle_connection(stream: TcpStream, settings: &Config) -> Result<(), std::io::Error> {
    println!("Connection established from {}", stream.peer_addr().unwrap());

    let remote = proxy_connection(&stream, settings)?;

    println!(
        "Connection from {} proxied successfully",
        stream.peer_addr()?
    );

    pump_streams(stream, remote)
}

// Returns the connected remote stream instead of just Ok(())
fn proxy_connection(_client: &TcpStream, settings: &Config) -> Result<TcpStream, std::io::Error> {
    let remotes: Vec<config::Value> = settings.get_array("remote_addr").unwrap();
    let proxy_port = settings.get_int("remote_port").unwrap();

    match remotes.get(0) {
        Some(value) => {
            let remote = TcpStream::connect(format!("{}:{}", value, proxy_port))?;
            Ok(remote)
        }
        None => {
            eprintln!("Error creating proxy connection");
            Err(io::Error::new(io::ErrorKind::ConnectionAborted, "no remote_addr configured"))
        }
    }
}

fn pump_streams(client: TcpStream, remote: TcpStream) -> Result<(), std::io::Error> {
    let mut client_read = client.try_clone()?;
    let mut client_write = client;

    let mut remote_read = remote.try_clone()?;
    let mut remote_write = remote;

    // client -> remote, on its own thread
    let client_to_remote = thread::spawn(move || -> io::Result<()> {
        io::copy(&mut client_read, &mut remote_write)?;
        remote_write.shutdown(std::net::Shutdown::Write).ok();
        Ok(())
    });

    // remote -> client, on the current thread
    let result = io::copy(&mut remote_read, &mut client_write)
        .map(|_| ())
        .and_then(|_| {
            client_write.shutdown(std::net::Shutdown::Write).ok();
            Ok(())
        });

    // Wait for the other direction to finish too
    let other_result = client_to_remote.join().expect("client->remote thread panicked");

    result.and(other_result)
}