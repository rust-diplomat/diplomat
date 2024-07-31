#include <stdio.h>
#include <stdarg.h>

#include "Wrapper.h"
#include "TestingStruct.h"
#include "diplomat_runtime.h"

// -------------------------------------------------------------------------- callbacks that will be passed to Rust

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

// ---------------------------------------------------------------- boilerplate for passing all the callbacks through to Rust
int32_t get_int_with_no_args_wrap(void* unused) {
    return get_int_with_no_args();
}
static struct DiplomatCallback_int32_t diplomat_callback_wrapper__get_int_with_no_args = { .run_callback = get_int_with_no_args };

int32_t callback_wrap(void* unused, int32_t x) {
    return callback(x);
}
static struct DiplomatCallback_int32_t diplomat_callback_wrapper__callback = { .run_callback = callback_wrap };

void multiarg_callback_wrap(void* unused, int32_t arg1, char* arg2) {
    multiarg_callback(arg1, arg2);
}
static struct DiplomatCallback_void diplomat_callback_wrapper__multiarg_callback = { .run_callback = multiarg_callback_wrap };

void mod_array_cb_wrap(void* unused, char* bytes) {
    mod_array_cb(bytes);
}
static struct DiplomatCallback_void diplomat_callback_wrapper__mod_array_cb = { .run_callback = mod_array_cb_wrap };

void no_arg_cb_wrap(void* unused) {
    no_arg_cb();
}
static struct DiplomatCallback_void diplomat_callback_wrapper__no_arg_cb = { .run_callback = no_arg_cb_wrap };

int32_t deal_with_struct_wrap(void* unused, TestingStruct ts) {
    return deal_with_struct(ts);
}
static struct DiplomatCallback_int32_t diplomat_callback_wrapper__deal_with_struct = { .run_callback = deal_with_struct_wrap };

// -------------------------------------------------------------------------------------- main
int main() {
    int32_t res = Wrapper_test_multi_arg_callback(diplomat_callback_wrapper__callback, 5);
    printf("Result: %d\n", res);

    Wrapper_test_multiarg_void_callback(diplomat_callback_wrapper__multiarg_callback);

    Wrapper_test_mod_array(diplomat_callback_wrapper__mod_array_cb);

    res = Wrapper_test_no_args(diplomat_callback_wrapper__no_arg_cb);
    printf("Got %d back from Rust\n", res);

    res = Wrapper_test_cb_with_struct(diplomat_callback_wrapper__deal_with_struct);
    printf("Got %d back from summing the struct fields\n", res);

    res = Wrapper_test_multiple_cb_args(diplomat_callback_wrapper__get_int_with_no_args, diplomat_callback_wrapper__callback);
    printf("And now the result of combining the results of 2 callbacks: %d\n", res);
}