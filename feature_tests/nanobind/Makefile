.DEFAULT_GOAL := test
.PHONY: build test

ALL_CPP_HEADERS := $(wildcard include/*.h) $(wildcard include/*.hpp) $(wildcard tests/*.hpp)
ALL_RUST := $(wildcard ../src/*.rs)
NANOBIND_LIB := nanobind/include/nanobind/nanobind.h
PYTHON_INCLUDE := $(shell python3 -c "from sysconfig import get_config_var; print(get_config_var(\"INCLUDEPY\"))")
PYTHON_LIBDIR := $(shell python3 -c "from sysconfig import get_config_var; print(get_config_var(\"LIBDIR\"))") 

CXX?=clang++-14

$(ALL_RUST):

$(ALL_CPP_HEADERS):

NANOBIND_LIB: 
    git clone https://github.com/wjakob/nanobind.git --depth=1 -b "v2.4.0" -q

../../target/debug/libdiplomat_feature_tests.a: $(ALL_RUST)
	cargo build

./tests/structs.out: ../../target/debug/libdiplomat_feature_tests.a $(ALL_HEADERS) src/somelib_ext.cpp
	$(CXX) -std=c++20 ./src/somelib_ext.cpp -I ../cpp/include -I nanobind/include -I ${PYTHON_INCLUDE} ../../target/debug/libdiplomat_feature_tests.a ${PYTHON_LIBDIR} -lpython3 -lstdc++ -ldl -lpthread -lm -g -o ./tests/feature_tests.pyd

test: ./tests/structs.out
	./tests/structs.out
