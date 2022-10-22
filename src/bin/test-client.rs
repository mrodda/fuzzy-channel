use std::{
    io::{Error, Read, Write},
    net::{TcpStream}
};

fn run() -> Result<(), Error> {
    let mut client = TcpStream::connect("127.0.0.1:7878")?;
    client.write_fmt(format_args!("Hi."))?;
    Ok(())
}

fn main() {
    println!("Start");
    if let Err(e) = run() {
        println!("Error: {:?}", e)
    } else {
        println!("End")
    }
}