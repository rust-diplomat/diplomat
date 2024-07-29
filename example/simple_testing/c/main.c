#include <stdio.h>

#include "Wrapper.h"
#include "diplomat_runtime.h"

int32_t callback(int32_t x) {
    return x + 1;
}

void multiarg_callback(int32_t arg1, char* arg2) {
    printf("Here are some args from Rust: %d, followed by %s", arg1, arg2);
}

int main() {
    DiplomatCallback cb_wrapper = {
        callback,
    };
    int32_t res = Wrapper_test_rust_fn(&cb_wrapper, 5);
    printf("Result: %d\n", res);

    DiplomatCallback multiarg_callback_wrapper = {
        multiarg_callback,
    };
    Wrapper_test_multiarg_callback(&multiarg_callback_wrapper);
}