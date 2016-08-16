extern crate mio;
extern crate tokio;

use mio::timer::Builder as MioTimerBuilder;
use tokio::reactor::*;
use tokio::udp::UdpSocket;
use tokio::util::timer::Timer;
use std::io;
use std::net::SocketAddr;
use std::time::Duration;

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
        println!("conn tick");
        let mut buf: [u8; 1024] = [0; 1024];

        match self.socket.recv_from(&mut buf) {
            Ok((bytes_read, addr)) => {
                match self.socket.send_to(&buf, &addr) {
                    Ok(_) => {
                        println!("send ok");
                        Ok(Tick::WouldBlock)
                    }
                    Err(e) => {
                        if e.kind() == io::ErrorKind::WouldBlock {
                            println!("send WouldBlock");
                            Ok(Tick::WouldBlock)
                        } else {
                            println!("send error");
                            Ok(Tick::Final)
                        }
                    }
                }
            }
            Err(e) => {
                if e.kind() == io::ErrorKind::WouldBlock {
                    println!("recv WouldBlock");
                    Ok(Tick::WouldBlock)
                } else {
                    println!("recv error");
                    Ok(Tick::Final)
                }
            }
        }
    }
}

struct GameTimer {
    timer : Timer<()>,
//    conn: Connection,
}

impl GameTimer {
    fn new(delay_from_now: Duration) -> GameTimer {
        let mut mio_timer = MioTimerBuilder::default()
            .tick_duration(delay_from_now)
            .build();

        // testing if adding more timeouts to the underlying timer will work
        mio_timer.set_timeout(Duration::from_millis(1000), ());
        mio_timer.set_timeout(Duration::from_millis(1000), ());
        mio_timer.set_timeout(Duration::from_millis(1000), ());

        let timer = Timer::watch(mio_timer).unwrap();

        GameTimer { timer: timer }
    }
}

impl Task for GameTimer {
    fn tick(&mut self) -> io::Result<Tick> {
        println!("timer");

        // idea is to add another timeout here so tick is called again
        self.timer.set_timeout(Duration::from_millis(1000), ());

        Ok(Tick::WouldBlock)
    }
}

fn main() {
    let reactor = Reactor::default().unwrap();

    reactor.handle().oneshot(move || {
        let socket = UdpSocket::bind(&"127.0.0.1:7777".parse().unwrap()).unwrap();
        let conn = Connection::new(socket);
        schedule(conn).unwrap();
        schedule(GameTimer::new(Duration::from_millis(1000))).unwrap();
    });

    reactor.run().unwrap();
}
