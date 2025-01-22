.DEFAULT_GOAL := test
.PHONY: build test

ALL_HEADERS := $(wildcard include/*.h) $(wildcard include/*.hpp) $(wildcard tests/*.hpp)
ALL_RUST := $(wildcard ../src/*.rs)

CXX?=clang++-16

$(ALL_RUST):

$(ALL_HEADERS):

../../target/debug/libdiplomat_feature_tests.a: $(ALL_RUST)
	cargo build

./tests/structs.out: ../../target/debug/libdiplomat_feature_tests.a $(ALL_HEADERS) ./tests/structs.cpp
	$(CXX) -std=c++20 ./tests/structs.cpp ../../target/debug/libdiplomat_feature_tests.a -lstdc++ -ldl -lpthread -lm -g -o ./tests/structs.out

./tests/result.out: ../../target/debug/libdiplomat_feature_tests.a $(ALL_HEADERS) ./tests/result.cpp
	$(CXX) -std=c++17 ./tests/result.cpp ../../target/debug/libdiplomat_feature_tests.a -ldl -lpthread -lm -g -o ./tests/result.out

./tests/option.out: ../../target/debug/libdiplomat_feature_tests.a $(ALL_HEADERS) ./tests/option.cpp
	$(CXX) -std=c++17 ./tests/option.cpp ../../target/debug/libdiplomat_feature_tests.a -ldl -lpthread -lm -g -o ./tests/option.out

./tests/attrs.out: ../../target/debug/libdiplomat_feature_tests.a $(ALL_HEADERS) ./tests/attrs.cpp
	$(CXX) -std=c++17 ./tests/attrs.cpp ../../target/debug/libdiplomat_feature_tests.a -ldl -lpthread -lm -g -o ./tests/attrs.out

./tests/callback.out: ../../target/debug/libdiplomat_feature_tests.a $(ALL_HEADERS) ./tests/callback.cpp
	$(CXX) -std=c++17 ./tests/callback.cpp ../../target/debug/libdiplomat_feature_tests.a -ldl -lpthread -lm -g -o ./tests/callback.out

test: ./tests/structs.out ./tests/result.out ./tests/option.out ./tests/attrs.out ./tests/callback.out
	./tests/structs.out
	./tests/result.out
	./tests/option.out
	./tests/attrs.out
	./tests/callback.out
