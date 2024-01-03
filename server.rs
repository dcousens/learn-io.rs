use std::io::ErrorKind::{WouldBlock};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

struct Client {
	stream: TcpStream,
	dropped: bool,
}

struct Message {
	from: usize,
	data: Vec<u8>,
}

fn main() -> std::io::Result<()> {
	let listener = TcpListener::bind("0.0.0.0:5000")?;
	listener.set_nonblocking(true)?; // FIXME: uses 100% CPU

	let mut clients = Vec::new();
	loop {
		// accept newcomers
		loop {
			let (stream, address) = match listener.accept() {
				Ok (result) => result,
				Err (e) if e.kind() == WouldBlock => { break },
				Err (e) => { return Err(e) }
			};

			println!("listen: {:?}", address);
			stream.set_nonblocking(true)?;
			clients.push(Client{
				stream: stream,
				dropped: false
			});
		}

		// an interim buffer
		let mut buffer = vec![0; 1024]; // use vec to prevent stack overflow

		// receive messages
		let mut outgoing = Vec::new();
		let mut i = 0;
		for client in &mut clients {
			i = i + 1;

			let read = match client.stream.read(&mut buffer[..]) {
				Ok (bytes) => buffer.get(0..bytes).unwrap(),
				Err (e) if e.kind() == WouldBlock => { continue }
				Err (e) => {
					eprintln!("stream.read(err): {:?}", e);
					continue
				}
			};

			if read.is_empty() { // dropped?
				client.dropped = true;
				println!("stream.read(dropped)");
				continue;
			}

			println!("stream.read(from: {}, bytes: {}): {}", i, read.len(), std::str::from_utf8(&read).unwrap());

			outgoing.push(Message{
				from: i,
				data: read.to_vec(),
			});
		}

		// broadcast messages
		let mut i = 0;
		for client in &mut clients {
			i = i + 1;

			for message in &outgoing {
				// skip messages from ourselves
				if message.from == i { continue }

				let written = match client.stream.write(&message.data[..]) {
					Ok(bytes) => bytes,
					Err (e) if e.kind() == WouldBlock => { continue }
					Err (e) => {
						eprintln!("stream.write(err): {:?}", e);
						continue
					}
				};

				if written == 0 { // dropped?
					client.dropped = true;
					println!("stream.write(dropped)");
					continue;
				}

				println!("stream.write(from: {}, to: {}, bytes: {})", message.from, i, written);
			}
		}

		// remove dropped clients
		let before = clients.len();
		clients.retain(|client| !client.dropped);
		let after = clients.len();
		if before > after {
			println!("dropped {} clients", before - after);
		}
	}
}
