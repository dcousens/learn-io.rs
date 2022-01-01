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
			match listener.accept() {
				Ok((stream, address)) => {
					println!("listen: {:?}", address);
					stream.set_nonblocking(true)?;
					clients.push(Client{
						stream: stream,
						dropped: false
					});
				}
				Err(ref e) if e.kind() == WouldBlock => { break; }
				Err(e) => {
					println!("listen(err): {:?}", e);
					return Ok(());
				}
			}
		}

		// an interim buffer
		let mut buffer = [0; 1024];

		// receive messages
		let mut outgoing = Vec::new();
		let mut i = 0;
		for client in &mut clients {
			i = i + 1;

			let read = client.stream.read(&mut buffer[..]);
			match read {
				Ok(bytes) => {
					if bytes == 0 { // dropped?
						client.dropped = true;
						println!("stream.read(dropped)");
						continue;
					}

					let data = buffer.get(0..bytes).unwrap();
					println!("stream.read(from: {}, bytes: {}): {}", i, bytes, std::str::from_utf8(&data).unwrap());

					outgoing.push(Message{
						from: i,
						data: data.to_vec(),
					});
				}
				Err(ref e) if e.kind() == WouldBlock => {}
				Err(e) => { println!("stream.read(err): {:?}", e); }
			}
		}

		// broadcast messages
		let mut i = 0;
		for client in &mut clients {
			i = i + 1;

			for message in &outgoing {
				if message.from == i {
					continue; // skip messages from ourselves
				}

				let written = client.stream.write(&message.data[..]);
				match written {
					Ok(bytes) => {
						if bytes == 0 { // dropped?
							client.dropped = true;
							println!("stream.write(dropped)");
							continue;
						}

						println!("stream.write(from: {}, to: {}, bytes: {})", message.from, i, bytes);
					}
					Err(ref e) if e.kind() == WouldBlock => {} // TODO: maybe blocking for write?
					Err(e) => { println!("stream.write(err): {:?}", e); }
				}
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
