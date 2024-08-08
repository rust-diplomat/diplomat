.DEFAULT_GOAL := main
.PHONY: build main

ALL_HEADERS := $(wildcard *.h)
ALL_RUST := $(wildcard ../src/*.rs)

CC=gcc

FORCE:

$(ALL_HEADERS):

../target/debug/libsomelib.a: FORCE
	cargo build

a.out: ../target/debug/libsomelib.a $(ALL_HEADERS) main.c
	$(CC) main.c ../target/debug/libsomelib.a -ldl -lpthread -lm -g

build: a.out

main: build
	./a.out
