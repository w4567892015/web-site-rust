MAKEFLAGS += --silent
ENV=${shell cat .env}

APP_DIR=apps
LIB_DIR=libs

NAME=$$name
OPTS=$$opts

new.app:
	cargo new "${APP_DIR}/${NAME}" --vcs none

new.lib:
	cargo new "${LIB_DIR}/${NAME}" --lib --vcs none

start.dev:
	cargo watch -x run -p ${NAME}

test:
	cargo test --lib --bins --tests
