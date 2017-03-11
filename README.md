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

1. With no timeout (calling `get_url`).

2. With a timeout just on `send` (calling `get_url_with_timeout_1`).

3. With a timeout on the whole thing (called `get_url_with_timeout_2`).

If you take out 3, it all compiles and runs.
What can I change about 3 to make that work too?

I've [asked Stackoverflow](http://stackoverflow.com/questions/42730169/fill-a-string-buffer-from-a-thread) for help,
and using the suggestions there, I have 3 solutions,
all involving Arc + Mutex.

<ol start="4">
  <li>Use an Arc&lt;Mutex&lt;String>> inside the function, then return a copy of the string (`get_url_with_timeout_3`).

  <li>Use an Arc&lt;Mutex&lt;String>> inside the function, then return it, and take the lock outside (`get_url_with_timeout_4`).

  <li>Create an Arc&lt;Mutex&lt;String>> outside the function, pass it as an argument, and afterwards take a lock on it (`get_url_with_timeout_5`).
</ol>

I like 5 for not requiring a copy, and I like 6 even more for letting the caller control the buffer initialization.

The latest commit on this repo now shows the invalid approach as commented-out,
and working code for the three solutions.


