/*Take in a TCP connection, terminate that TCP connection, and then proxy it to a different endpoint 
Will require TCP connection, echo that traffic back to the endpoint, receive from the sent to endpoint
and send back to original whatever*/

/*Could also be possible to give an enum, or struct, or something of the endpoints to send to, and then implement some version of 
load balancing, or health checks, or something. */

mod connections;

use std::{net::{TcpListener}};

use config::Config;

fn main() {
    let read_settings: Result<Config, config::ConfigError> = Config::builder()
    .add_source(config::File::with_name("settings"))
    .add_source(config::Environment::with_prefix("APP"))
    .build();

    let settings = match read_settings {
        Ok(read_settings) =>  read_settings,
        Err(error) => {eprintln!("Error occurred: {}", error);
        std::process::exit(1);}
    };


    let host: String = settings.get_string("bind_address").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: String = settings.get_string("listen_port").unwrap_or_else(|_| 8080.to_string());

    let bind_address: String = format!("{}:{}", host, port);

    println!("Socket bound on {}", bind_address);
    
    let listener: Result<TcpListener, std::io::Error> = TcpListener::bind(bind_address);

    match listener {
        Ok(connection) => for stream in connection.incoming(){
            let stream = stream.unwrap();
            connections::handle_connection(stream, &settings).unwrap();
        },
        Err(error) => println!("An error occurred creating listener: {}", error),
    };

}


