# Picard

Try to build a SPA in Rust with these frameworks:

- [seed](https://seed-rs.org/)
- [warp](https://github.com/seanmonstar/warp)

```bash
cargo clean && cargo build && ./target/debug/picard
```

```bash
curl -i http://localhost:3030/todos
```

## Seed Quickstart

**To get started:**

- Clone this repo: `git clone https://github.com/seed-rs/seed-quickstart.git`
- If you don't have Rust and cargo-make installed, [Download it](https://www.rust-lang.org/tools/install), and run the following commands:

```bash
rustup update
rustup target add wasm32-unknown-unknown
cargo install cargo-make
```

Run `cargo make build` in a terminal to build the app, and `cargo make serve` to start a dev server
on `127.0.0.1:8000`.

If you'd like the compiler automatically check for changes, recompiling as
needed, run `cargo make watch` instead of `cargo make build`.

**Deploy**

1. Run `cargo make build release`
2. Upload `index.html` and `pkg` to your web server

## Frontend Stuff

- [Bourbon SASS lib](https://www.bourbon.io/docs/latest/)