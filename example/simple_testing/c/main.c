#include <stdio.h>

#include "Wrapper.h"
#include "diplomat_runtime.h"

int32_t callback(int32_t x) {
    return x + 1;
}

int main() {
    DiplomatCallback* cb_wrapper = diplomat_callback_create_for_c(callback);
    int32_t res = DiplomatCallback_call_test_rust_fn(cb_wrapper);
    printf("Result: %d\n", res);
}