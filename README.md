# ChRusty

A chat application written in Rust using websockets.

## Planned features
- Snappy, multi-threaded UI / network logic
- Account creation
- Friends list / friend behavior
- Multi-threaded message request handling
- Efficient storage using an SQL database.

## Libraries used
- [ws](https://docs.rs/ws/latest/ws/)
- [flume](https://docs.rs/flume/latest/flume/)

## Building
1. Install Rust and Cargo
2. `cd chrusty`
3. `cargo build`
