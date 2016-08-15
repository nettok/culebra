extern crate tokio;

use tokio::reactor::*;
use tokio::udp::UdpSocket;
use std::io;
use std::net::SocketAddr;

struct Connection {
    socket: UdpSocket,
}

impl Connection {
    fn new(socket: UdpSocket) -> Connection {
        Connection { socket: socket }
    }
}

impl Task for Connection {
    fn tick(&mut self) -> io::Result<Tick> {

        let mut buf: [u8; 1024] = [0; 1024];

        match self.socket.recv_from(&mut buf) {
            Ok((bytes_read, addr)) => {
                match self.socket.send_to(&buf, &addr) {
                    Ok(_) => Ok(Tick::Final),
                    Err(e) => {
                        if e.kind() == io::ErrorKind::WouldBlock {
                            Ok(Tick::WouldBlock)
                        } else {
                            Ok(Tick::Final)
                        }
                    }
                }
            }
            Err(e) => {
                if e.kind() == io::ErrorKind::WouldBlock {
                    Ok(Tick::WouldBlock)
                } else {
                    Ok(Tick::Final)
                }
            }
        }
    }
}

fn main() {
    let reactor = Reactor::default().unwrap();

    reactor.handle().oneshot(move || {
        let socket = UdpSocket::bind(&"127.0.0.1:7777".parse().unwrap()).unwrap();
        schedule(Connection::new(socket));
    });

    reactor.run().unwrap();
}
