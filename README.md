# Rust Actix-web Starter

A high-performance, container-ready web service template built with Rust and the Actix-web framework. This project serves as a robust starting point for developing fast and reliable web applications.

## ✨ Features

- **Static File Serving**: Serves static assets from the `/assets` directory.
- **Health Check**: Includes a `/healthchecker` endpoint for monitoring.
- **Environment-based Configuration**: Easily configure the app using a `.env` file.
- **Structured Logging**: Integrated `env_logger` for clear and filterable logs.
- **Containerized**: Comes with a `Dockerfile` for easy containerization and deployment.
- **Benchmarking Ready**: Includes a `benchmark` setup with `docker-compose` and `k6`.

## 🛠️ Tech Stack

- **Backend**: [Rust](https://www.rust-lang.org/) with [Actix-web](https://actix.rs/)
- **Web Server**: [Nginx](https://www.nginx.com/) (see `config/nginx.conf`)
- **Containerization**: [Docker](https://www.docker.com/) & [Docker Compose](https://docs.docker.com/compose/)

## 📋 Prerequisites

- [Rust Toolchain](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/) (for containerized workflows)

## 🚀 Getting Started

1.  **Clone the repository:**
    ```bash
    git clone <repository-url>
    cd test_web_site
    ```

2.  **Configure the environment (optional):**
    Create a `.env` file in the root directory by copying the example below. If not provided, the server defaults to `0.0.0.0:3000`.
    ```env
    # .env
    HOST=127.0.0.1
    PORT=8080
    WORKER_NUM=4
    ```

3.  **Run the development server:**
    ```bash
    cargo run
    ```
    The application will be available at `http://127.0.0.1:8080` (or your configured host and port).

## 🏗️ Building for Production

To create an optimized release build, run:
```bash
cargo build --release
```
The compiled binary will be located at `target/release/test_web_site`.

## 🐳 Docker Usage

You can build and run the application using Docker for a consistent and isolated environment.

1.  **Build the Docker image:**
    ```bash
    docker build -t test-web-site .
    ```

2.  **Run the Docker container:**
    ```bash
    docker run -p 8080:3000 -d test-web-site
    ```
    The application will be accessible at `http://localhost:8080`.

## ⚡ Benchmarking

The project includes a simple benchmarking setup using k6.

1.  **Navigate to the benchmark directory:**
    ```bash
    cd benchmark
    ```
2.  **Start the services:**
    ```bash
    docker-compose up -d
    ```
3.  **Run the test script:**
    (Requires [k6](https://k6.io/docs/getting-started/installation/) to be installed locally)
    ```bash
    k6 run run.js
    ```
4.  **Shutdown the services:**
    ```bash
    docker-compose down
    ```

## 📂 Project Structure

```
/
├── assets/           # Static files (HTML, CSS, JS)
├── benchmark/        # k6 and docker-compose for performance testing
├── config/           # Nginx configuration
├── src/              # Rust source code
│   ├── health/       # /health endpoint module
│   ├── index/        # / endpoint module
│   └── main.rs       # Application entry point
├── Cargo.toml        # Rust package definition and dependencies
└── Dockerfile        # Container build instructions
```

## 🌐 Endpoints

- `GET /api`: Serves the api endpoint.
- `GET /assets/*`: Serves the main `index.html` from the `assets` directory.
- `GET /healthchecker`: Returns a `200 OK` status for health checks.
