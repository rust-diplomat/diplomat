SHELL := /bin/bash
.DEFAULT_GOAL := test
.PHONY: build test

test:
	python3 -m venv .venv
	source .venv/bin/activate; \
	uv pip install . --reinstall \
	uv pip install pytest; \
	python3 -m pytest
	