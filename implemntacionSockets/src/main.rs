use std::net::UdpSocket;
use std::thread;
use std::str;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:34254")?;
    let mut buf = [0; 2048];

    loop {
      match socket.recv_from(&mut buf) {
        Ok((buf_size, src_addr)) => {
          thread::spawn(move || {
            let buf = &mut buf[..buf_size];
            let req_msg = str::from_utf8(&buf).unwrap();
            println!("{:}", "=".repeat(80));
            println!("buffer size: {:?}", buf_size);
            println!("src address: {:?}", src_addr);
            println!("request message: {:?}", req_msg);
          });
        },
        Err(e) => {
          println!("couldn't recieve request: {:?}", e);
        }
      }
    }
}