# Stage0-Profile API




A simple Rust API built with [Axum](https://docs.rs/axum/).

---

## Requirements

- **Rust** (1.82+ recommended)  
  Install via [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
````

* **Cargo** (comes with Rust)
* **Docker** (optional, if running in a container)

---

## Dependencies

Rust dependencies are listed in `Cargo.toml` and will be installed automatically:

```toml
axum = "0.8.6"
tokio = { version = "1.48.0", features = ["macros", "rt-multi-thread"] }
reqwest = { version = "0.12.24", features = ["json", "rustls-tls"] }
serde = "1.0.228"
serde_json = "1.0.145"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["fmt", "env-filter"] }
```

---

## Environment Variables

No environment variables are required for basic local running.

---

## Run Locally

1. Clone the repository:

```bash
git clone https://github.com/yourusername/stage0-profile.git
cd stage0-profile
```

2. Build the project:

```bash
cargo build --release
```

3. Run the API:

```bash
cargo run
# or
./target/release/stage0-profile
```

4. The API will listen on **`localhost:3000`** by default.

5. Test the endpoint:

```bash
curl http://localhost:3000/me 
```

---

## Run with Docker (optional)

```bash
docker build -t axum-api .
docker run -p 3000:3000 axum-api
```


