# Rust Cheat Sheet
1. Sort of hot testing & reloading: `cargo watch -x check`
2. Linting: `cargo clippy`
3. Package auditing: `cargo audit`
4. Formatting: `cargo fmt`
5. Expanding macros: `cargo expand`
6. Run the server: `cargo run`

# Personal notes

## 3.3
1. The `Responder` traits main job is just to allow things to be converted into `HttpResponse`
2. All asynchronous programming in Rust is built on the `Futures` trait - particularly, they expose a `poll` method that has to be called for this to make progress
   1. This is why `main` cannot be an async function - who is going to call poll on it?
