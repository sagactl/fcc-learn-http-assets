# Communicating on the web

Instagram would be pretty terrible if you have to manually copy your photos to your friend's phone when you wanted to share them. Modern applications need to be able to communicate information *between devices* over the internet.

* Gmail doesn't just store your emails in variables on your computer, it stores them on computers in their data centers
* You don't lose your slack messages if you drop your computer in a lake, those messages exist on Slack's [servers](https://en.wikipedia.org/wiki/Web_server)

## How does web communication work?

When two computers communicate with each other, they need to use the same rules. An English speaker can't communicate verbally with a Japanese speaker, similarly, two computers need to speak the same language to communicate.

This "language" that computers use is called a [protocol](https://en.wikipedia.org/wiki/Communication_protocol). The most popular protocol for web communication is [HTTP](https://developer.mozilla.org/en-US/docs/Web/HTTP/Overview), which stands for Hypertext Transfer Protocol.

## Assignment 

I wrote the `get_user_data()` function for you. It retrieves users from [JSONPlaceholder](https://jsonplaceholder.typicode.com), a free fake API for testing and prototyping, via HTTP using the [reqwest](https://docs.rs/reqwest) crate.

I also wrote a `log_users()` function for you, call it with the `users` variable we are getting back from `get_user_data()` as a parameter. 

## Tip

Notice how none of the data that is logged to the console was generated within our code! That's because the data we retrieved is being sent over the internet from their servers via HTTP. Don't worry, we'll explain more about that later.

## Running

```bash
cargo run
```
