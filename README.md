# Modulo Server

The Modulo Server is a backend server developed in Rust, designed to handle high-performance computing tasks with a focus on safety, concurrency, and maintainability. This server is a key component of Project VCHAT, and it also serves as a flexible and modifiable codebase for various use cases.

## Table of Contents
- [Features](#features)
- [Why Rust?](#why-rust)
- [Why Was the Modulo Server Created?](#why-was-the-modulo-server-created)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Features

- **Memory Safety**: Rust's ownership system ensures memory safety without needing a garbage collector, preventing common bugs like null pointer dereferencing and buffer overflows.
- **High Performance**: Provides performance comparable to C and C++ by avoiding runtime overhead, making it ideal for systems programming and performance-critical applications.
- **Concurrency**: Rust's fearless concurrency model allows safe and efficient multi-threading, making it easier to build scalable and concurrent applications.
- **Modern Tooling**: Equipped with a rich set of tools, including Cargo (Rust's package manager), a build system, and a comprehensive testing framework.
- **Maintainability**: Rust’s syntax and rich type system make the code easier to read, maintain, and extend.
- **Community and Ecosystem**: Backed by a strong and growing community with a wide array of libraries and frameworks.

## Why Rust?

Rust was chosen as the programming language for the Modulo Server due to its unique combination of high performance, safety, and concurrency. The language's focus on preventing common bugs at compile-time ensures that the server runs reliably and efficiently. Additionally, Rust’s modern tooling and strong community support made it a perfect fit for this project.

### Benefits of Using Rust for the Modulo Server

1. **High Performance**: The server can handle numerous requests efficiently, ensuring minimal latency.
2. **Reliability**: Fewer runtime errors thanks to Rust's compile-time safety checks.
3. **Concurrency Support**: Efficient handling of multiple client requests simultaneously.
4. **Maintainability**: Easily maintain and extend the codebase as needed.

## Why Was the Modulo Server Created?

1. **For Project VCHAT**: The Modulo Server was developed as a crucial component of Project VCHAT, providing a reliable backend to support the project's communication needs.
2. **Easily Modifiable Source Code**: Designed to be flexible, allowing developers to adapt and extend the codebase to meet evolving requirements.
3. **To Pass Time After School**: The project also served as a way to hone programming skills and experiment with new technologies after school hours.

## Installation

To install and run the Modulo Server, ensure that you have Rust installed on your machine. Then, clone the repository and build the project:

```bash
# Clone the repository
git clone https://github.com/stupid759/modulo-src.git

# Navigate into the project directory
cd modulo-src

# Build the project
cargo build --release
