# Rust Cheat Sheet
1. Sort of hot testing & reloading: `cargo watch -x check`
2. Linting: `cargo clippy`
3. Package auditing: `cargo audit`
4. Formatting: `cargo fmt`
5. Expanding macros: `cargo expand`
   1. For specific files `cargo expand --test health_check`
6. Run the server: `cargo run`

# Personal notes

## 3.3
1. The `Responder` traits main job is just to allow things to be converted into `HttpResponse`
2. All asynchronous programming in Rust is built on the `Futures` trait - particularly, they expose a `poll` method that has to be called for this to make progress
   1. This is why `main` cannot be an async function - who is going to call poll on it?

## 3.4
1. In the previous implementation in 3.3, the `run` method creates the server and calls await on it. The `main` method which calls run has to await in order to unwrap the future (ie. it is a future in a future)
2. The test implementation here gets to just call `tokio::spawn` on the server which spins it up as a background task instead of running the test as a server

## 3.5
1. Port 0 triggers an OS scan for whatever available port
