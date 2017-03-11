# Rust: fill a buffer in a thread

With `hyper`, I need to make an HTTP connection and read the results.
I want to wrap the whole thing in a timeout,
so I start a thread
and use [`recv_timeout`](https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html#method.recv_timeout) to wait for it.

Wrapping just the [`send`](https://hyper.rs/hyper/v0.9.8/hyper/client/request/struct.Request.html#method.send) works,
but I want to also wrap [`read_to_string`](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string).
Unfortunately I get a compiler error:

    error[E0477]: the type `[closure@src/main.rs:53:25: 58:4 tx:std::sync::mpsc::Sender<std::result::Result<u16, MyAppError>>, url:std::string::String, buf:&mut std::string::String]` does not fulfill the required lifetime
      --> src/main.rs:53:11
       |
    53 |   let t = thread::spawn(move || {
       |           ^^^^^^^^^^^^^
       |
       = note: type must outlive the static lifetime

How can I pass the buffer to the thread,
let it fill it,
and then print out the buffer back on the main thread?

(This is Rust 1.15.1.)

This repository shows three examples for getting the webpage:

1. With no timeout.

2. With a timeout just on `send`.

3. With a timeout on the whole thing.

If you take out 3, it all compiles and runs.
What can I change about 3 to make that work too?



