default: server client cat

client: client.rs
	rustc $< -o $@

server: server.rs
	rustc $< -o $@

cat: cat.rs
	rustc $< -o $@
