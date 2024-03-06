## Learning Axum

Following along [Rust Production Coding - Web App Series by Jeremy Chone.](https://youtube.com/playlist?list=PL7r-PXl6ZPcCTTxjmsb9bFZB9i01fAtI7&si=E55wdDxIr6JOzNHk)
Adding documentation along the way.

## Starting the DB

```sh
# Start postgresql server docker image:
docker run --rm --name pg -p 5432:5432 \
   -e POSTGRES_PASSWORD=welcome \
   postgres:15

# (optional) To have a psql terminal on pg.
docker exec -it -u postgres pg psql

# (optional) For pg to print all sql statements.
ALTER DATABASE postgres SET log_statement = 'all';
```

## DEV

1.  Install cargo-watch:
    ```sh
    cargo install cargo-watch
    ```
2.  Use two terminals to develop, in the first one run:
    ```sh
    cargo watch -q -c -w src/ -w .cargo/ -x run
    ```
    Re-compiles every time you change anything in the /src directory
3.  In the second use:
    ```sh
    cargo watch -q -c -w examples/ -x "run --example quick_dev"
    ```
    Runs a test every time you change anything in the /examples directory

## Unit Test (watch)

```sh
cargo watch -q -c -x "test -- --nocapture"

# Specific test with filter.
cargo watch -q -c -x "test model::task::tests::test_create -- --nocapture"
```
