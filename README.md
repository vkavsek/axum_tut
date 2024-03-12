## Learning Axum

Following along [Rust Production Coding - Web App Series by Jeremy Chone.](https://youtube.com/playlist?list=PL7r-PXl6ZPcCTTxjmsb9bFZB9i01fAtI7&si=E55wdDxIr6JOzNHk)
Adding documentation along the way.

## Starting the DB

### Start postgresql server docker image:

```sh
docker run --rm --name pg -p 5432:5432 \
   -e POSTGRES_PASSWORD=welcome \
   postgres:15
```

### (optional) To have a psql terminal on pg.

```sh
docker exec -it -u postgres pg psql
```

### (optional) For pg to print all sql statements.

```sql
ALTER DATABASE postgres SET log_statement = 'all';
```

## Run the web server

```sh
cargo run -p web-server
```

## For development

1.  Install cargo-watch:
    ```sh
    cargo install cargo-watch
    ```
2.  Use two terminals to develop, in the first one run:
    ```sh
    cargo watch -q -c -w src/ -w .cargo/ -x "run -p web-server"
    ```
    Re-compiles every time you change anything in the _/src_ directory
3.  In the second use:
    ```sh
    cargo watch -q -c -w examples/ -x "run --example quick_dev"
    ```
    Runs the `quick_dev` example every time you change anything in the _/examples_ directory

## Unit Testing

Watch:

```sh
cargo watch -q -c -x "test -- --nocapture"
```

Specific test with filter:

```sh
cargo watch -q -c -x "test -p lib-core model::task::tests::test_create -- --nocapture"
```
