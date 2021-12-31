default: server client

client: client.rs
	rustc $< -o $@

server: server.rs
	rustc $< -o $@
