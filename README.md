# echo.rs demo
A `std` library demonstration of IO patterns in Rust.

WARNING: This is a work in progress! It is not idiomatic rust.
It's a learning exercise.

## Things to do
- `std::thread` instead of `set_nonblocking`,  using [`std::sync`](https://doc.rust-lang.org/std/sync/index.html)
- [use `metal`](https://github.com/tokio-rs/mio)


## How to test
In a terminal
``` bash
> ./server
```

Then, in two other terminals
``` bash
> ./client alice
> ./client bob
```

Type into either terminals to send messages, hit ENTER (new line) to spin the `stream.read` loop
``` bash
> ./client alice
hello
stdin.read(bytes: 7): hello

stream.write(25)

stream.read(bytes: 19): bob@1641002882: hi
```

(and in the other terminal)
``` bash
> ./client bob

stream.read(bytes: 25): alice@1641002874: hello

hi
stdin.read(bytes: 3): hi

stream.write(19)
```

## LICENSE [MIT](LICENSE)
