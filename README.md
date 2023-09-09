## Learning Axum

Use two terminals to develop, in the first one run:
	* `cargo watch -q -c -w src/ -x run`	
Re-compiles everytime you change anything in the /src directory
	
In the second use:
	* `cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"`
Runs a test everytime you change anything in the /tests directory
