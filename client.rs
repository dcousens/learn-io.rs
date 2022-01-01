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
	let mut buffer = [0; 1024];

	loop {
		let read = stream.read(&mut buffer[..]);
		match read {
			Ok(bytes) => {
				if bytes == 0 { break } // dropped

				let data = buffer.get(0..bytes).unwrap();
				println!("stream.read(bytes: {}): {}", bytes, std::str::from_utf8(&data).unwrap());
			}
			Err(ref e) if e.kind() == WouldBlock => {}
			Err(e) => { println!("stream.read(err): {:?}", e); }
		}

		let mut string = String::new();
		let result = io::stdin().read_line(&mut string);
		match result {
			Ok(length) => {
				if length <= 1 { continue } // empty/new line to loop

				println!("stdin.read(bytes: {}): {}", length, string);
				let when = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
				let message = format!("{:}@{:?}: {:}", identifier, when, string);

				let written = stream.write(message.as_bytes())?;
				println!("stream.write({})", written);
			}
			Err(e) => { println!("stdin.read(err): {:?}", e); }
		}
	}

	Ok(())
}
