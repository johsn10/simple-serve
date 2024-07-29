pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[macro_export]
macro_rules! run_body {
    ($body:expr, $($input_value:expr)*) => {
        thread::spawn(move || {
            $body($( $input_value.clone() )*);
        })
    };
}

#[macro_export]
macro_rules! simple_serve {
    ( (port: $port:literal), $( ( $method:expr, $url:expr ) ( $($input_value:expr),* ) => $body:expr ),+ ) => {
        use std::thread;
        use tiny_http::{self, StatusCode};


        thread::spawn(move || {
            use simple_serve::run_body;
            let server = tiny_http::Server::http("0.0.0.0:8000").unwrap();

            println!("Listening on port: {}", $port);

            loop {
                let request = match server.recv() {
                    Ok(rq) => rq,
                    Err(e) => {
                        println!("Error {}", e);
                        break;
                    }
                };
                let url = request.url();
                if vec![$($url,)+].contains(&url) {
                    $(
                        if $url == url {
                            let response = $body($($input_value,)*);
                            match request.respond(response) {
                                Ok(_) => {},
                                Err(e) => { println!("Error {}", e); break},
                            };
                            continue
                        }
                    )+
                } else {
                    let response = Response::from_string("404 Not Found")
                        .with_status_code(404);
                    match request.respond(response) {
                        Ok(_) => {},
                        Err(e) => { println!("Error {}", e); break },
                    };
                }

            }
        })
    };
}

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}
