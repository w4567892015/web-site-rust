MAKEFLAGS += --silent
ENV=${shell cat .env}

APP_DIR=apps
LIB_DIR=libs

NAME=$$name
OPTS=$$opts

help:
	echo "[Rust Application Tasks]"
	echo "- make new.app name=<NAME>"
	echo "- make new.lib name=<NAME>"
	echo "- make start.dev name=<NAME>"
	echo "- make release name=<NAME>"
	echo "- make test name=<NAME>"
	echo "- make clean"

new.app:
	cargo new "${APP_DIR}/${NAME}" --vcs none

new.lib:
	cargo new "${LIB_DIR}/${NAME}" --lib --vcs none

start.dev:
	cargo watch -x run -p ${NAME} | bunyan

clean:
	cargo clean

release:
	cargo build --release -p ${NAME}

test:
	cargo test --lib --bins --tests
