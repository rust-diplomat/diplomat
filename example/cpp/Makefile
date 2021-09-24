.DEFAULT_GOAL := main
.PHONY: build main

ALL_HEADERS := $(wildcard *.h) $(wildcard *.hpp)
ALL_RUST := $(wildcard ../src/*.rs)

CXX=g++

$(ALL_RUST):

$(ALL_HEADERS):


../../target/debug/libexample.a: $(ALL_RUST)
	cargo build

a.out: ../../target/debug/libexample.a $(ALL_HEADERS) main.cpp
	$(CXX) -std=c++20 main.cpp ../../target/debug/libexample.a -ldl -lpthread -lm -g

build: a.out

main: build
	./a.out
