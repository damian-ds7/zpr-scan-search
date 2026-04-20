.PHONY: lint fmt dev build test test-rust test-python

PYTHON_SRC = python/
rust  ?= false
python ?= false

ifeq ($(rust)-$(python), false-false)
  _rust   = true
  _python = true
else
  _rust   = $(rust)
  _python = $(python)
endif

dev:
	uv run maturin develop

build:
	uv run maturin build --release

lint:
ifeq ($(_rust), true)
	cargo clippy -- -D warnings
endif
ifeq ($(_python), true)
	uv run ruff check $(PYTHON_SRC)
endif

fmt:
ifeq ($(_rust), true)
	cargo fmt
endif
ifeq ($(_python), true)
	uv run ruff format $(PYTHON_SRC)
endif

test:
ifeq ($(_rust), true)
	cargo test
endif
ifeq ($(_python), true)
	uv run maturin develop --skip-install
	uv run pytest
endif
