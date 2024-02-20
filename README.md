## Learning Axum

Following along [Rust Axum Full Course by Jeremy Chone.](https://youtu.be/XZtlD_m59sM?si=RjQ9oBaOAN_DLEDh)
Adding documentation along the way.

## Starting the DB

```sh
# Start postgresql server docker image:
docker run --rm --name pg -p 5432:5432 \
   -e POSTGRES_PASSWORD=welcome \
   postgres:15

# (optional) To have a psql terminal on pg.
# In another terminal (tab) run psql:
docker exec -it -u postgres pg psql

# (optional) For pg to print all sql statements.
# In psql command line started above.
ALTER DATABASE postgres SET log_statement = 'all';
```

## DEV

1. Install cargo-watch:
   `cargo install cargo-watch`
2. Use two terminals to develop, in the first one run:
   `cargo watch -q -c -w src/ -w .cargo/ -x run` Re-compiles every time you change anything in the /src directory
3. In the second use:
   `cargo watch -q -c -w examples/ -x "run --example quick_dev"` Runs a test everytime you change
   anything in the /examples directory

## Unit Test (watch)

```sh
cargo watch -q -c -x "test -- --nocapture"

# Specific test with filter.
cargo watch -q -c -x "test model::task::tests::test_create -- --nocapture"
```
