use std::io::{BufRead, BufReader, Result as IoResult, Write};
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) -> IoResult<()> {
    let reader = BufReader::new(&stream);

    let http_request = reader
        .lines()
        .map(|line| line.expect("expected a line of string"))
        .take_while(|line| !line.is_empty())
        .collect::<Vec<String>>();
    println!("Request: {http_request:#?}");

    let (status_code, html_body) = match http_request[0].as_str() {
        "GET / HTTP/1.1" => {
            ("HTTP/1.1 200 OK", include_str!("webserver/hello_rust.html"))
        }
        "GET /sleep HTTP/1.1" => {
            std::thread::sleep(std::time::Duration::from_secs(10));
            ("HTTP/1.1 200 OK", include_str!("webserver/hello_rust.html"))
        }

        _ => ("HTTP/1.1 404 NOT FOUND", include_str!("webserver/404.html")),
    };

    let length = html_body.len();
    let response =
        format!("{status_code}\r\nContent-Length: {length}\r\n\r\n{html_body}");
    stream.write_all(response.as_bytes())?;

    Ok(())
}

pub fn server() -> IoResult<()> {
    let listener = TcpListener::bind("127.0.0.1:7372")?;

    for stream in listener.incoming() {
        let stream = stream?;
        handle_connection(stream)?;
    }
    Ok(())
}
