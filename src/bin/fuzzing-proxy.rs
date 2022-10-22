use std::{
    io::{Error, Read},
    net::{TcpListener}
};
use log::{info, error};

fn run() -> Result<(), Error> {
    // Server.
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    // Listen for a connection.
    let (mut stream, _addr) = listener.accept()?;
    // Read a string.
    let mut buf = String::new();
    let _size = stream.read_to_string(&mut buf)?;
    info!("Read '{}'", buf);
    Ok(())
}
fn main() {
    env_logger::init();
    info!("Start");
    if let Err(e) = run() {
        error!("Error {:?}", e);
    }
    info!("End")
}
