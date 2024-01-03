use std::io;
use std::io::ErrorKind::{WouldBlock};
use std::io::prelude::*;
use std::net::TcpStream;
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;

fn main() -> std::io::Result<()> {
	let mut stream = TcpStream::connect("127.0.0.1:5000")?;
	stream.set_nonblocking(true)?;

	let mut identifier = String::from("unnamed");
	for argument in env::args() {
		identifier = argument;
	}

	// an interim buffer
	let mut buffer = vec![0; 1024]; // use vec to prevent stack overflow

	loop {
		let read = match stream.read(&mut buffer[..]) {
			Ok (bytes) => buffer.get(0..bytes).unwrap(),
			Err (e) if e.kind() == WouldBlock => { continue }
			Err (e) => {
				eprintln!("stream.read(err): {:?}", e);
				continue
			}
		};

		if read.is_empty() { break } // dropped
		println!("stream.read(bytes: {}): {}", read.len(), std::str::from_utf8(&read).unwrap());

		let mut string = String::new();
		let length = match io::stdin().read_line(&mut string) {
			Ok (length) => length,
			Err (e) => { return Err(e) }
		};

		if length <= 1 { continue } // empty/new line to loop

		println!("stdin.read(bytes: {}): {}", length, string);
		let when = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
		let message = format!("{:}@{:?}: {:}", identifier, when, string);
		let written = stream.write(message.as_bytes())?;
		println!("stream.write({})", written);
	}

	Ok(())
}
