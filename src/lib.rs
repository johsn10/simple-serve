pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[macro_export]
macro_rules! simple_serve {
    ( (port: $port:literal), $( ( $method:expr, $route:expr ) ( $($input_value:expr),* ) => $body:expr ),+ ) => {
        use std::net::{TcpListener, TcpStream};
        use std::thread;

        println!("Listening on port: {}", $port);
        thread::spawn(move || {
            $(
                let listener = TcpListener::bind(format!("127.0.0.1:{}", $port)).unwrap();
                for stream in listener.incoming() {
                    match stream {
                        Ok(mut stream) => {
                            let input_values = $($input_value.clone())*;
                            let return_value = $body(&mut stream, input_values);
                        },
                        Err(e) => println!("{}", e),
                    }
                }

                let method: Method = $method;
                let route: &str = $route;
            )+
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
