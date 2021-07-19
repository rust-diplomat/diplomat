.DEFAULT_GOAL := main
.PHONY: build main

ALL_HEADERS := $(wildcard api.h)
ALL_RUST := $(wildcard ../src/*.rs)

$(ALL_RUST):

$(ALL_HEADERS):


../../target/debug/libexample.a: $(ALL_RUST)
	cargo build

a.out: ../../target/debug/libexample.a $(ALL_HEADERS) main.c
	gcc main.c ../../target/debug/libexample.a -ldl -lpthread -lm -g

build: a.out

main: build
	./a.out
