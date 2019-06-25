use std::thread;
use std::io::{self, Write};
use std::net::{TcpListener};

static RESPONSE: &'static [u8] = include_bytes!("response.txt");

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1024")?;
    for stream in listener.incoming() {
        thread::spawn(move || {
            match stream.ok() {
                Some(mut stream) => stream.write(RESPONSE).ok(),
                None => None,
            }
        });
    }
    Ok(())
}
