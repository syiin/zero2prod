# Commands Cheat Sheet
1. Sort of hot testing & reloading: `cargo watch -x check`
2. Linting: `cargo clippy`
3. Package auditing: `cargo audit`
4. Formatting: `cargo fmt`
5. Expanding macros: `cargo expand`
   1. For specific files `cargo expand --test health_check`
6. Run the server: `cargo run`
7. Starting a database: `SKIP_DOCKER=true ./scripts/init_db.sh`
8. To create & run migrations:
   ```
   sqlx migrate add create_subscriptions_table
   export DATABASE_URL=postgres://app:secret@localhost:5432/newsletters
   sqlx migrate run
   ```
9. Test submitting a subscriber: `curl -i -X POST -d 'email=thomas_mann@hotmail.com&name=Tom' http://127.0.0.1:8000/subscriptions`
10. Generate cached queries for compilation without a database: `cargo sqlx prepare --workspace -- --all-targets`
11. Run with network: `docker run -p 8000:8000 --network my-network zero2prod-network zero2prod`
12. Digital ocean deploy:
    1.  First time: `doctl apps create --spec spec.yaml`
    2.  Deploy: `doctl apps update 82c8a437-1920-4230-b5b7-f446f9a5eff3 --spec=spec.yaml`
    3.  APP ID from: `doctl apps list`
    4.  Migrate database: `DATABASE_URL=postgres://newsletter:{PASSWORD}@app-1f6b10e0-d334-40f0-9800-ee4444479a98-do-user-14672112-0.e.db.ondigitalocean.com:25060/newsletter sqlx migrate run`

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

## 3.7
1. actix-web's handlers calls the from_request methods inside the arguments passed before they even get passed to the handler
   1. This allows the handler to deal with strongly typed arguments
   2. The data (ie. FormData) handles the data itself
   3. So, what's happening is when the email or name is missing, FormData wrapped in serde will return 400 without us needing to explicitly handle it in the handler
2. Serde stands for serialisation/deserialisation
3. Misc aside, reminder to self:
   1. `#[derive(serde::Deserialize)]` is saying to implement the serde::Deserialize trait for `FormData`
   2. `impl<T> Serialize for Vec<T>` is an implementation of Serialize for `Vec`, NOT a definition of the trait

## 3.8
1. Personal reflection: API integration tests agnostic to the underlying implementation (ie. instead of querying the database within the test to check for side effects, query a GET endpoint to inspect the data after the fact)
2. `export DATABASE_URL=postgres://app:secret@localhost:5432/newsletter && sqlx migrate run`
3. `lib.rs` and `main.rs` are special files as defined in the `Cargo.toml` - ie. that's why lib is where public modules are specified
   1. Then the `routes/mod.rs` defines what is exposed there
4. The Turbofish operator defines generics in a function: `fn pair<T, U>(first: T, second: U) -> (T, U)` and used by: `pair::<i32, &str>(42, "hello")`

## 3.9
1. `HttpServer::new` takes a closure and invokes this function whenever a new worker is created
   1. This is why it has to be cloneable
2. Arc - Atomic Reference Counter
   1. `Arc<T>` is always cloneable and passes a pointer to this single instance 
   2. web::Data wraps the connection in an Arc and passes to every worker
3. actix-web uses a type-map (ie. `{HashMap<TypeId, Box<dyn Any>>}`)
   1. When a request is received, it looks for the TypeId of the parameter
   2. If there are multiple parameters of the same type, you have to wrap them in 2 different structs eg. `MainDatabaseConnection` and `LoggingDatabaseConnection`
4. The Rust compiler enforces that there can only be one active mutable reference at a time and so, sqlx's execute asks for a mutable pgConnection so it can be sure that it and only it can run queries over the same connection
5. `move` inside the Http::Server::new makes the closure take ownership of `db_pool` instead of just borrowing it - otherwise, the db_pool might outlive the closure

## 3.10
1. The `Executor` trait imported from `sqlx` is needed to make `connection.execute()` work - otherwise the trait isn't in scope
2. "Observability is about being able to ask arbitrary questions about
your environment without — and this is the key part — having to know
ahead of time what you wanted to ask."

## 4.5
1. The exiting a span (`->`) involves temporarily putting a span down (eg. polled a future). Closing a span (`-`) involves finally when the span is dropped (ie. Rust's RAII system)
2. HRTBs are written using for<'a> syntax. The for<'a> declares that the following trait bound or function is valid for all lifetimes 'a
   1. `where` this is about constraits, eg. generic `Sink` implements the `Send` + `Sync` traits
   2. `for<'a>MakeWriter<'a>` means, `Sink` must be able to make a writer for any possible lifetime `<'a>`
   3. `'static` means `Sink` must have no non-static references

## 5.38
1. Reminders on how to optimise docker things:
   1. Use and update your .dockerignore
   2. Stage your docker builds