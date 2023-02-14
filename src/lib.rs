use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;

/// A WebSocket echo server
pub fn start_server() {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
        match stream {
            Ok(stream) => {
                spawn(move || handle_request(stream));
            }
            Err(e) => print_error(e),
        }
    }
}

fn handle_request(stream: std::net::TcpStream) {
    let mut buf = [0; 512];
    if let Err(e) = stream.peek(&mut buf) {
        return print_error(e);
    }
    println!("Received: {}", String::from_utf8_lossy(&buf));

    if buf.starts_with(b"GET /socket HTTP/1.1") {
        handle_websocket(stream);
    } else {
        handle_http_request(stream);
    }
    match accept(stream) {
        Ok(ws) => handle_websocket(ws),
        Err(e) => return print_error(e),
    }
}

fn handle_websocket(stream: std::net::TcpStream) {
    let mut websocket = accept(stream)?;
    loop {
        let msg = websocket.read_message()?;

        // We do not want to send back ping/pong messages.
        if msg.is_binary() || msg.is_text() {
            websocket.write_message(msg)?;
        }
    }
}

fn print_error<T: std::fmt::Display>(e: T) {
    println!("Error: {e}");
}
