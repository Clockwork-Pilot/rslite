# RSLITE is a port of Sqlite3 on rust.

Our goal is seamless integration of SQLite into Rust applications, keeping it as close to the original SQLite architecture as possible and ensuring it passes existing sqlite tests.

We’re iteratively porting SQLite to Rust by first generating an unsafe Rust equivalent. Then we link the existing SQLite tests and make changes in small steps, verifying the tests at each iteration. This way, each modification is checked and validated through a continuous development loop.

At this point, we haven’t actually changed the interface, so it remains fully compatible with the original SQLite. But since we plan to change it in the future as the project evolves, we’ve already ported some of these tests to Rust. This way, when we modify the library, we’ll also be adjusting the tests suite accordingly.

# Build and test

run `./build_all.sh` to build & test

## Run just a separate test, after tests built
cd /sqlite && ./rustfixture test/capi3.test
