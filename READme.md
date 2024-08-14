# Rust Multi-Threaded Web Server

This project implements a simple multi-threaded web server in Rust. It uses a thread pool to handle incoming HTTP requests concurrently. The server is capable of serving static files and handling basic GET requests.

## Features

- Multi-threaded architecture utilizing Rust's standard library
- Thread pool implementation for efficient resource management
- Handles static file serving and basic HTTP GET requests

## Getting Started

### Prerequisites

- Rust programming language (ensure it's installed: [Install Rust](https://www.rust-lang.org/tools/install))

### Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/blaze-d83/multi-threaded-rs.git
    ```
2. Navigate to the Project directory:
   ```sh
   cd multi-threaded-rs
   ```
3. Build  / Run the Project:
```sh
cargo build --release
cargo run
```

### Usage

1. Run the Server
2. Access the server in your browser at `http://127.0.0.1:7878`



