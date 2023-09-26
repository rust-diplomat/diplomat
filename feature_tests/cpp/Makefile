.DEFAULT_GOAL := test
.PHONY: build test

ALL_HEADERS := $(wildcard *.h) $(wildcard *.hpp) $(wildcard tests/*.hpp)
ALL_RUST := $(wildcard ../src/*.rs)

CXX=g++-10

FORCE:

$(ALL_HEADERS):

../../target/debug/libdiplomat_feature_tests.a: FORCE
	cargo build

./tests/structs.out: ../../target/debug/libdiplomat_feature_tests.a $(ALL_HEADERS) ./tests/structs.cpp
	$(CXX) -std=c++17 ./tests/structs.cpp ../../target/debug/libdiplomat_feature_tests.a -ldl -lpthread -lm -g -o ./tests/structs.out

./tests/result.out: ../../target/debug/libdiplomat_feature_tests.a $(ALL_HEADERS) ./tests/result.cpp
	$(CXX) -std=c++17 ./tests/result.cpp ../../target/debug/libdiplomat_feature_tests.a -ldl -lpthread -lm -g -o ./tests/result.out

./tests/option.out: ../../target/debug/libdiplomat_feature_tests.a $(ALL_HEADERS) ./tests/option.cpp
	$(CXX) -std=c++17 ./tests/option.cpp ../../target/debug/libdiplomat_feature_tests.a -ldl -lpthread -lm -g -o ./tests/option.out

test: ./tests/structs.out ./tests/result.out ./tests/option.out
	./tests/structs.out
	./tests/result.out
	./tests/option.out
