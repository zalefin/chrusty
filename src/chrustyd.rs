use ws::{listen, CloseCode};
use chrusty::Packet;

fn main() {
    let host = String::from("127.0.0.1");
    let port = 5000;

    let packet = Packet{
        opcode: 1,
        src: 1,
        dst: 2,
        body: format!("hello there!")
    };
    println!("chrustyd!");
    packet.print();

    let handle = listen(format!("{host}:{port}"), |out| {

        move |msg| {
            println!("Received ->{msg}");
            out.send(msg)
        }

    }).expect("Incoming connection failed.");

}
