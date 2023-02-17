#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![warn(absolute_paths_not_starting_with_crate,elided_lifetimes_in_paths,explicit_outlives_requirements,keyword_idents,let_underscore_drop,macro_use_extern_crate,meta_variable_misuse,missing_abi,missing_copy_implementations,missing_docs,non_ascii_idents,noop_method_call,pointer_structural_match,rust_2021_incompatible_closure_captures,rust_2021_incompatible_or_patterns,rust_2021_prefixes_incompatible_syntax,rust_2021_prelude_collisions,single_use_lifetimes,trivial_casts,trivial_numeric_casts,unreachable_pub,unsafe_code,unsafe_op_in_unsafe_fn,unstable_features,unused_crate_dependencies,unused_extern_crates,unused_import_braces,unused_lifetimes,unused_macro_rules,unused_qualifications,unused_results,unused_tuple_struct_fields,variant_size_differences)]


use std::io::Write;
use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;

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

    let res = if buf.starts_with(b"GET /socket HTTP/1.1") {
        handle_websocket(stream);
    } else {
        handle_http_request(stream);
    };
}

fn handle_websocket(stream: std::net::TcpStream) {
    let mut websocket = accept(stream);
    match websocket {
        Ok(mut websocket) => {
            handle_websocket_messages(&mut websocket);
        }
        Err(e) => {
            print_error(e);
        }
    }
}

fn handle_http_request(mut stream: std::net::TcpStream) {
    let content_length = DUMMY_PAGE.len();
    let response = format!("HTTP/1.1 200 OK\r
Content-Type: text/html\r
Content-Length: {content_length}\r
\r
{DUMMY_PAGE}");
    if let Err(e) = stream.write_all(response.as_bytes()){
        print_error(e);
    }

}

fn handle_websocket_messages(websocket: &mut tungstenite::WebSocket<std::net::TcpStream>) {
    loop {
        let msg = websocket.read_message();
        match msg {
            Ok(msg) => {
                println!("Received: {}", msg);
                websocket.write_message(msg).unwrap();
            }
            Err(e) => {
                print_error(e);
                break;
            }
        }
    }
}

fn print_error<T: std::fmt::Display>(e: T) {
    println!("Error: {e}");
}

const DUMMY_PAGE: &str = r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>WebSocket Echo Server</title>
    </head>
    <body>
        <h1>WebSocket Echo Server</h1>
        <p>Open the browser console to see the output.</p>
        <script>
            const socket = new WebSocket("ws://localhost:9001/socket");
            socket.addEventListener("open", function (event) {
                socket.send("Hello Server!");
            });
            socket.addEventListener("message", function (event) {
                console.log("Message from server ", event.data);
            });
        </script>
    </body>
"#;
