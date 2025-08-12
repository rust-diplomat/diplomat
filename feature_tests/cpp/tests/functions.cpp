#include <iostream>
#include "../include/diplomat_functions.hpp"
#include "../include/nested/ns/diplomat_nested_ns_functions.hpp"
#include "../include/ns/diplomat_ns_functions.hpp"
#include "assert.hpp"

int main(int argc, char* argv[]) {
    simple_assert_eq("Nested Namespaced Func", nested::ns::Renamednested_ns_fn(true), false);
    simple_assert_eq("Namespaced Func", ns::Renamedfree_func_test(0), 5);
}