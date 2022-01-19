// From: https://rust-lang-nursery.github.io/rust-cookbook/net/server.html
use std::io::{Error, Read};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};

fn main() -> Result<(), Error> {
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 0);
    let listener = TcpListener::bind(socket)?;
    let port = listener.local_addr()?;
    println!(
        "Listening on http://{}, access this port to end the program",
        port
    );
    match listener.accept() {
        //block  until requested
        Ok((mut _socket, addr)) => {
            println!("Connection received! http://{:?} is sending data.", addr);
            let mut input = String::new();
            let _ = _socket.read_to_string(&mut input)?;
            println!("{:?} says {}", addr, input);
        }
        Err(e) => println!("couldn't get client: {:?}", e),
    };
    Ok(())
}
