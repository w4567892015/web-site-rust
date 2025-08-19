# test-rust-web-site

這是一個使用 Rust 和 Actix-web 框架開發的簡單 Web 服務。

## 專案描述

本專案提供一個基本的 HTTP 伺服器，它會在根路徑 (`/`) 回應 "Hello, World!"。專案結構包含了 Docker 設定，使其易於容器化部署，並包含效能測試的設定。

## 功能

- 一個 `/` 端點，回傳 "Hello, World!"
- 使用 `.env` 檔案進行環境變數設定
- 使用 `env_logger` 進行日誌紀錄
- 提供 `Dockerfile` 以進行容器化
- 提供 `docker-compose.yaml` 用於效能測試

## 環境需求

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/) (選用，用於容器化)
- [Docker Compose](https://docs.docker.com/compose/install/) (選用，用於效能測試)

## 如何開始

### 1. 複製專案

```bash
git clone <repository-url>
cd test_web_site
```

### 2. (選用) 設定環境變數

您可以建立一個 `.env` 檔案來設定 `HOST` 和 `PORT`。如果未設定，預設將使用 `0.0.0.0:3000`。

```
# .env
HOST=127.0.0.1
PORT=8080
```

### 3. 在本地端執行

```bash
cargo run
```

應用程式將會啟動在 `http://<HOST>:<PORT>`。

## 測試

執行 Rust 的標準測試指令：

```bash
cargo test
```

## 建置

若要建置用於生產環境的執行檔：

```bash
cargo build --release
```

執行檔將會位於 `./target/release/test_web_site`。

## Docker

您也可以使用 Docker 來建置和執行此應用程式。

### 1. 建置 Docker 映像

```bash
docker build -t test_web_site .
```

### 2. 執行 Docker 容器

```bash
docker run -p 3000:3000 -d test_web_site
```

應用程式將會在 `http://localhost:3000` 上提供服務。

## 效能測試

專案中包含了使用 `docker-compose` 的基本效能測試設定。

1.  進入 `performance` 目錄：
    ```bash
    cd performance
    ```
2.  啟動服務：
    ```bash
    docker-compose up -d
    ```
3.  執行測試腳本 (假設您已經安裝了 k6)：
    ```bash
    k6 run run.js
    ```
4.  完成後關閉服務：
    ```bash
    docker-compose down
    ```
