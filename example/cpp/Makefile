.DEFAULT_GOAL := test
.PHONY: build test

ALL_HEADERS := $(wildcard *.h) $(wildcard *.hpp) $(wildcard tests/*.hpp)
ALL_RUST := $(wildcard ../src/*.rs)

CXX?=clang++-16

$(ALL_RUST):

$(ALL_HEADERS):


../../target/debug/libdiplomat_example.a: $(ALL_RUST)
	cargo build

./tests/fixeddecimal.out: ../../target/debug/libdiplomat_example.a $(ALL_HEADERS) ./tests/fixeddecimal.cpp
	$(CXX) -std=c++17 ./tests/fixeddecimal.cpp ../../target/debug/libdiplomat_example.a -ldl -lpthread -lm -g -o ./tests/fixeddecimal.out

./tests/fixeddecimal-20.out: ../../target/debug/libdiplomat_example.a $(ALL_HEADERS) ./tests/fixeddecimal.cpp
	$(CXX) -std=c++20 ./tests/fixeddecimal.cpp ../../target/debug/libdiplomat_example.a -ldl -lpthread -lm -g -o ./tests/fixeddecimal-20.out


test: ./tests/fixeddecimal.out ./tests/fixeddecimal-20.out
	./tests/fixeddecimal.out
	./tests/fixeddecimal-20.out
