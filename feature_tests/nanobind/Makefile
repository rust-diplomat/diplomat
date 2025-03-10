SHELL := /bin/bash
.DEFAULT_GOAL := test
.PHONY: build test

test:
	cd somelib
	uv venv
	source .venv/bin/activate; \
	uv pip install . --reinstall; \
	uv pip install pytest; \
	python3 -m pytest