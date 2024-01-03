use std::io;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
	let mut stdin = io::stdin();
	let mut stdout = io::stdout();

	// an interim buffer
	let mut buffer = vec![0; 1024]; // use vec to prevent stack overflow

	loop {
		let iobuffer = buffer.get_mut(0..).unwrap();
		let read = match stdin.read(iobuffer) {
			Ok (bytes) => iobuffer.get(0..bytes).unwrap(),
			Err (e) => { return Err(e) }
		};

		if read.is_empty() { return Ok(()) }

		loop {
			match stdout.write_all(read) {
				Ok (_) => break,
				Err (e) => { return Err(e) }
			};
		}
	}
}
