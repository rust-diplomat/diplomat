.DEFAULT_GOAL := main
.PHONY: build main

ALL_HEADERS := $(wildcard *.h)
ALL_RUST := $(wildcard ../src/*.rs)

CC=gcc

FORCE:

$(ALL_HEADERS):

../../target/debug/libdiplomat_example.a: FORCE
	cargo build

a.out: ../../target/debug/libdiplomat_example.a $(ALL_HEADERS) main.c
	$(CC) main.c ../../target/debug/libdiplomat_example.a -ldl -lpthread -lm -g

build: a.out

main: build
	./a.out
