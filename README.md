# Test Project (Rust)

這是一個使用 Rust 建立的 Monorepo 專案，包含一個高效能的 Web 應用程式和共用函式庫。

## 📋 先決條件 (Prerequisites)

在開始之前，請確保您已安裝以下工具：

- [Rust Toolchain](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/) & [Docker Compose](https://docs.docker.com/compose/)
- [Make](https://www.gnu.org/software/make/)
- [Bunyan](https://www.npmjs.com/package/bunyan) (用於美化日誌輸出 `npm install -g bunyan`)

## 📂 工作區結構 (Workspace Structure)

本專案採用工作區架構，將不同的功能模組化：

```
/
├── apps/
│   └── web/          # Actix-web 網頁應用程式
├── libs/
│   └── otel/         # OpenTelemetry 共用函式庫
├── Makefile          # 專案指令
└── Cargo.toml        # 工作區設定
```

- **`apps/web`**: 一個使用 Actix-web 框架建置的高效能 Web 服務，已整合日誌、環境變數設定並包含容器化支援。
- **`libs/otel`**: 一個共用的函式庫，用於處理 OpenTelemetry (Tracing, Metrics, Logs) 的初始化與設定。

## 🚀 快速開始 (Getting Started)

1.  **啟動專案所需資源:**
    ```bash
    cd test_web_site/apps/web
    docker compose up
    ```

2.  **啟動開發伺服器:**
    使用 `make` 指令來啟動 `web` 應用程式的開發伺服器。它會自動監聽檔案變更並重新載入。
    ```bash
    make start.dev name=web
    ```
    應用程式將會在本機的 `http://127.0.0.1:8080` 上執行 (預設)。

## 🛠️ 主要指令 (Makefile Commands)

我們提供了一個 `Makefile` 來簡化常見的開發任務。

- `make start.dev name=<app_name>`
  啟動指定的應用程式，並在檔案變更時自動重載。
  ```bash
  # 範例
  make start.dev name=web
  ```

- `make release name=<app_name>`
  為指定的應用程式建立一個最佳化的產品級建置 (release build)。
  ```bash
  # 範例
  make release name=web
  ```

- `make test`
  執行工作區中所有的測試。

- `make new.app name=<app_name>`
  在 `apps` 目錄下建立一個新的應用程式模板。

- `make new.lib name=<lib_name>`
  在 `libs` 目錄下建立一個新的函式庫模板。

- `make clean`
  清除所有建置產物。
