use chrusty;
use ws::{connect, CloseCode};

fn main() {
    let host = String::from("127.0.0.1");
    let port = 5000;

    let handle = connect(format!("ws://{host}:{port}"), |out| {
        out.send("Hello from client").unwrap();

        move |msg| {
            println!("Got message -> {msg}");
            Ok(())
        }
    }).expect("Connection failed");
}

fn usage() {
    println!("./chrusty HOST PORT CLIENT_ID");
}
