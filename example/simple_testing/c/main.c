#include <stdio.h>

#include "Wrapper.h"
#include "TestingStruct.h"
#include "diplomat_runtime.h"

int32_t callback(int32_t x) {
    return x + 1;
}

void multiarg_callback(int32_t arg1, char* arg2) {
    printf("Here are some args from Rust: %d, followed by %s\n", arg1, arg2);
}

void mod_array_cb(char* bytes) {
    bytes[0] = 0x00;
}

void no_arg_cb() {
    printf("Calling the no arg CB from Rust\n");
}

int deal_with_struct(TestingStruct ts) {
    return ts.x + ts.y;
}

int get_int_with_no_args() {
    return 10;
}

int main() {
    DiplomatCallback cb_wrapper = {
        callback,
    };
    int32_t res = Wrapper_test_multi_arg_callback(&cb_wrapper, 5);
    printf("Result: %d\n", res);

    DiplomatCallback multiarg_cb_wrapper = {
        multiarg_callback,
    };
    Wrapper_test_multiarg_void_callback(&multiarg_cb_wrapper);

    DiplomatCallback mod_array_cb_wrapper = {
        mod_array_cb,
    };
    Wrapper_test_mod_array(&mod_array_cb_wrapper);

    DiplomatCallback no_args_cb_wrapper = {
        no_arg_cb,
    };
    res = Wrapper_test_no_args(&no_args_cb_wrapper);
    printf("Got %d back from Rust\n", res);

    DiplomatCallback deal_with_struct_cb_wrapper = {
        deal_with_struct,
    };
    res = Wrapper_test_cb_with_struct(&deal_with_struct_cb_wrapper);
    printf("Got %d back from summing the struct fields\n", res);

    // test_multiple_cb_args(f: impl Fn() -> i32, g: impl Fn(i32) -> i32) -> i32
    DiplomatCallback get_int_with_no_args_cb_wrapper = {
        get_int_with_no_args,
    };
    res = Wrapper_test_multiple_cb_args(&get_int_with_no_args_cb_wrapper, &cb_wrapper);
    printf("And now the result of combining the results of 2 callbacks: %d\n", res);
}