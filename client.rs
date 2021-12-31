use std::io::prelude::*;
use std::net::TcpStream;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> std::io::Result<()> {
	let mut stream = TcpStream::connect("127.0.0.1:5000")?;
	let my_identifier = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

	loop {
		let message = format!("{:?}: hello", my_identifier);
		let written = stream.write(message.as_bytes())?;
		println!("stream.write({})", written);

		let mut buffer = [0; 64];
		let read = stream.read(&mut buffer[..])?;
		if read == 0 { break } // dropped

		let data = buffer.get(0..read).unwrap();
		println!("stream.read({}): {}", read, std::str::from_utf8(&data).unwrap());
	}

	Ok(())
}
