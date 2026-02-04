# Rust's reqwest crate

In this course, we'll be using the [reqwest](https://docs.rs/reqwest) crate to make HTTP requests. We already used it in the last two assignments!

The `reqwest` crate provides both a blocking (synchronous) and async interface. Let's look at the blocking version:

## Using reqwest (blocking)

```rust
let client = reqwest::blocking::Client::new();
let response = client
    .get(url)
    .send()
    .expect("Failed to send request");

let data: Vec<User> = response.json().expect("Failed to parse JSON");
```

Let's break this down:

* `client` is the HTTP client that will make our requests
* `url` is the URL we are making a request to
* `.send()` actually sends the request and returns a `Result<Response, Error>`
* `.expect()` unwraps the Result, panicking with the message if there's an error
* `.json()` deserializes the response body into a Rust type using serde

## The Result type

In Rust, operations that can fail return a `Result<T, E>` type. Unlike JavaScript where errors might be thrown as exceptions, Rust makes you explicitly handle the possibility of failure.

```rust
// This won't compile - Result must be handled!
let response = client.get(url).send();
// response is Result<Response, Error>, not Response

// You need to handle the Result:
let response = client.get(url).send().expect("Failed");
// Now response is Response
```

## Assignment

Fix the bug in the code.

The problem is that we're not properly handling the `Result` returned by `.send()`. The code tries to call `.json()` on a `Result` instead of on the actual `Response`.

## Running

```bash
cargo run
```
