# Learn HTTP - Rust Edition

This is a Rust adaptation of the [Boot.dev Learn HTTP course](https://www.boot.dev/courses/learn-http).

The original course is available in JavaScript/TypeScript and Go. This adaptation brings the same concepts to Rust using the [reqwest](https://docs.rs/reqwest) crate for HTTP requests.

## Prerequisites

- Rust toolchain installed ([rustup.rs](https://rustup.rs))
- Basic understanding of Rust syntax

## Structure

Each chapter is organized into exercises:
- `readme.md` - The lesson content and assignment
- `src/main.rs` - The code with a bug or incomplete implementation
- `src/main_complete.rs` - The complete solution
- `expected.txt` - The expected output
- `Cargo.toml` - Dependencies for the exercise

## Running exercises

```bash
cd course-rust/1-why-http/exercises/1-web-communication
cargo run
```

## Chapters

1. **Why HTTP** - Introduction to web communication, requests/responses, URLs, and the client-server model

## Credits

- Original course by [Boot.dev](https://www.boot.dev)
- Rust adaptation by [@sagactl](https://github.com/sagactl)
