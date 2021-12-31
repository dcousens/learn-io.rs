# echo.rs demo
A `std` library only TCP server/client demonstration in Rust.

WARNING: This is a work in progress! It is not idiomatic rust.
It's a learning exercise.

## Things to do
- `std::thread` instead of `set_nonblocking`,  using [`std::sync`](https://doc.rust-lang.org/std/sync/index.html)


## How to test
In a terminal
```
./server
```

Then, in two other terminals
```
./client alice
./client bob
```

You should then see the two clients talking back-and-forth to each-other
```
stream.read(21): bob@1640926820: hello
stream.write(23)
...
```

(and in the other terminal)
```
stream.read(23): alice@1640926820: hello
stream.write(21)
...
```
