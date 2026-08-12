/*Take in a TCP connection, terminate that TCP connection, and then proxy it to a different endpoint 
Will require TCP connection, echo that traffic back to the endpoint, receive from the sent to endpoint
and send back to original whatever*/

use std::{fs, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}, thread, time::Duration};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream){
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
}

