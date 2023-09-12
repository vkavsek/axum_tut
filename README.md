## Learning Axum

Following along [Rust Axum Full Course by Jeremy Chone.](https://youtu.be/XZtlD_m59sM?si=RjQ9oBaOAN_DLEDh)
Adding documentation along the way.

1. Install cargo-watch: 
   `cargo install cargo-watch`
2. Use two terminals to develop, in the first one run:
	`cargo watch -q -c -w src/ -x run` Re-compiles everytime you change anything in the /src directory
3. In the second use:
	`cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"` Runs a test everytime you change
	anything in the /tests directory
