SHELL := bash
.ONESHELL:
	.SHELLFLAGS := -eu -o pipefail -c
.DELETE_ON_ERROR:
	MAKEFLAGS += --warn-undefined-variables
	MAKEFLAGS += --no-builtin-rules

lint: fmt
	cargo clippy --fix --allow-staged
.PHONY: lint

fmt:
	rustfmt crates/**/src/*.rs
.PHONY: fmt
