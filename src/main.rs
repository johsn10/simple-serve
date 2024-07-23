use simple_serve::{simple_serve, Method};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

fn main() {
    let state: Arc<Mutex<i32>> = Arc::from(Mutex::from(2));
    let ss_state = state.clone();

    simple_serve! {
        (port: 8000),
        (Method::Get, "/") (ss_state) => |stream: &mut TcpStream, state: Arc<Mutex<i32>>| {
            let response = format!("HTTP/1.1 200 OK\r\n\r\n");
            let mut buffer = [0; 50];

            stream.read(&mut buffer).unwrap();
            stream.write(response.as_bytes())
                .expect("Couldn't send data.");
            stream.flush().unwrap();

            println!("State: {:?}", state);
            println!("Request: {}", String::from_utf8_lossy(&buffer));
        }
    };
    loop {}
}
