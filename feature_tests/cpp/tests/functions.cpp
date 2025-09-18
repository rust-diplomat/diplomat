#include <iostream>
#include "../include/free_functions.hpp"
#include "../include/nested/ns/free_functions.hpp"
#include "../include/ns/free_functions.hpp"
#include "assert.hpp"

using namespace somelib;

int main(int argc, char* argv[]) {
    simple_assert_eq("Nested Namespaced Func", nested::ns::Renamednested_ns_fn(true), false);
    simple_assert_eq("Namespaced Func", ns::Renamedfree_func_test(0), 5);
}