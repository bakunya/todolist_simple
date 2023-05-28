use std::{thread};
use std::net::{TcpListener};
use http_handlers::{client::handle_client};
use database::connection::{connect as db_connect};

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

mod database {
    pub mod connection;
    pub mod todos {
        pub mod create;
        pub mod get_all;
        pub mod get_by_id;
        pub mod delete_by_id;
        pub mod update_by_id;
    }
}

#[tokio::main]
async fn main () {
    let pool = db_connect().await;

    match pool {
        Ok(res) => {
            println!("Database connected!");
            let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
            println!("Server listening on port 8080");

            for stream in listener.incoming() {
                let stream = stream.unwrap();
                let p = res.clone();
                thread::spawn(move || handle_client(stream, p));
            }
        },
        Err(er) => {
            print!("Failed to connect database {:?}", er)
        }
    };
}