use simple_serve::{simple_serve, Method};
use std::{fs::File, str::FromStr};
use tiny_http::{Header, Response};

fn main() {
    let state = 1;

    simple_serve! {
        (port: 8000),
        (Method::Get, "/") (state) => |state| {
            println!("State: {}", state);
            Response::from_string("Working!")
        },
        (Method::Get, "/home") () => || {
            let html = File::open("src/home.html").unwrap();
            let mut response: Response<File> = Response::from_file(html);
            response.add_header(Header::from_str("Content-Type: text/html;").unwrap());
            response
        }
    };
    loop {}
}
