/*Take in a TCP connection, terminate that TCP connection, and then proxy it to a different endpoint 
Will require TCP connection, echo that traffic back to the endpoint, receive from the sent to endpoint
and send back to original whatever*/

/*Could also be possible to give an enum, or struct, or something of the endpoints to send to, and then implement some version of 
load balancing, or health checks, or something. */

use std::{fs, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}, thread, time::Duration};

use config::Config;

fn main() {
    let read_settings = Config::builder()
    .add_source(config::File::with_name("settings"))
    .add_source(config::Environment::with_prefix("APP"))
    .build();

    let settings = match read_settings {
        Ok(read_settings) =>  read_settings,
        Err(error) => {eprintln!("Error occurred: {}", error);
        std::process::exit(1);}
    };

    let remotes = settings.get_array("remote_addr").unwrap();

    match remotes.get(0){
        Some(value) => println!("{}", value),
        None => println!("No value found"),
    };

    let host: String = settings.get_string("bind_address").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: String = settings.get_string("listen_port").unwrap_or_else(|_| 8080.to_string());

    let bind_address = format!("{}:{}", host, port);

    println!("Socket bound on {}", bind_address);
    
    let listener = TcpListener::bind(bind_address);

    match listener {
        Ok(connection) => for stream in connection.incoming(){
            let stream = stream.unwrap();
            handle_connection(stream).unwrap();
        },
        Err(error) => println!("An error occurred creating listener: {}", error),
    };

}

fn handle_connection(mut stream: TcpStream) -> Result<(), std::io::Error>{

    println!("Connection established from {}", stream.peer_addr().unwrap());

    let buf_read = BufReader::new(&stream);
    //let _http_request: Vec<_> = buf_read.lines().map(|result| result.unwrap()).take_while(|line| !line.is_empty()).collect();

    let request_line = buf_read
    .lines()
    .next()
    .unwrap()
    .unwrap();


    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")}

        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

        let contents = fs::read_to_string(filename).unwrap();
        let length = contents.len();
        
        let response = format!("{status_line}\r\n\
                    Content-Length: {length}\r\n\r\n\
                    {contents}");

            stream.write_all(response.as_bytes()).unwrap();

    stream.shutdown(std::net::Shutdown::Both)

}

