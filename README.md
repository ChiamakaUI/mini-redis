# RustKV – A Minimal Redis-Like Server in Rust

RustKV is a minimal in-memory key-value server written in Rust using Tokio.
It supports basic Redis-style commands over TCP and is designed as a learning project to explore:
- Async networking with Tokio
- Trait-based abstraction
- Interior mutability with RwLock
- Shared state using Arc
- Basic benchmarking techniques

## Features
- TCP server (127.0.0.1:8080)
- Concurrent clients via Tokio tasks
- Thread-safe shared store
- String keys and values only
- Supported commands:
  - `SET key value`
  - `GET key`
  - `DEL key`
  - `EXISTS key`
  - `PING`

## Architecture
### Core Components
1️⃣ Store
- Backed by HashMap<String, String>
- Wrapped in RwLock for concurrent reads
- Wrapped in Arc for shared ownership across tasks
```
Arc
 └── Store
      └── RwLock
           └── HashMap
```
2️⃣ Storage Trait
The store implements a Storage<T> trait:
```
pub trait Storage<T> {
    fn get(&self, key: &str) -> Option<T>;
    fn set(&self, key: String, val: T);
    fn delete(&self, key: &str) -> bool;
}
```
This abstraction allows alternative storage backends in the future.

3️⃣ Async Networking
Each client connection is handled by:
```
tokio::spawn(async move {
    handle_client(socket, store).await;
});
```
This enables concurrent clients sharing the same store.

## Installation
```
git clone https://github.com/YOUR_USERNAME/rustkv.git
cd rustkv
cargo run
```
Server starts on:
```
127.0.0.1:8080
```
## Usage
You can test with nc:
```
nc 127.0.0.1 8080
```

Then type:
```
SET name Ada
GET name
DEL name
```
