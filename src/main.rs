use std::{thread};
use std::net::{TcpListener};
use http_handlers::{client::handle_client};

mod http_handlers {
    pub mod get;
    pub mod put;
    pub mod post;
    pub mod error;
    pub mod delete;
    pub mod client;
}

mod utils {
    pub mod read_file;
    pub mod write_file;
}

mod types { 
    pub mod http;
    pub mod todo;
}

fn main () {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| handle_client(stream));
    }
}