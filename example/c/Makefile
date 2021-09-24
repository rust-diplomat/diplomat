.DEFAULT_GOAL := main
.PHONY: build main

ALL_HEADERS := $(wildcard *.h)
ALL_RUST := $(wildcard ../src/*.rs)

CC=gcc

$(ALL_RUST):

$(ALL_HEADERS):


../../target/debug/libexample.a: $(ALL_RUST)
	cargo build

a.out: ../../target/debug/libexample.a $(ALL_HEADERS) main.c
	$(CC) main.c ../../target/debug/libexample.a -ldl -lpthread -lm -g

build: a.out

main: build
	./a.out
