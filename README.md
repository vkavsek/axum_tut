## Learning Axum

Following along [Rust Axum Full Course by Jeremy Chone.](https://youtu.be/XZtlD_m59sM?si=RjQ9oBaOAN_DLEDh)
Adding documentation along the way.

1. Install cargo-watch:
   `cargo install cargo-watch`
2. Use two terminals to develop, in the first one run:
   `cargo watch -q -c -w src/ -w .cargo/ -x run` Re-compiles every time you change anything in the /src directory
3. In the second use:
   `cargo watch -q -c -w examples/ -x "run --example quick_dev"` Runs a test everytime you change
   anything in the /examples directory
