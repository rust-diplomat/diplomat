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

// -------------------------------------------------------------------------------------- main
int main() {
    DiplomatCallback_Wrapper_test_multi_arg_callback_f diplomat_callback_wrapper__callback =
        C_create_DiplomatCallback_Wrapper_test_multi_arg_callback_f(callback);
    int32_t res = Wrapper_test_multi_arg_callback(diplomat_callback_wrapper__callback, 5);
    printf("Result: %d\n", res);

    DiplomatCallback_Wrapper_test_multiarg_void_callback_f diplomat_callback_wrapper__multiarg_callback =
        C_create_DiplomatCallback_Wrapper_test_multiarg_void_callback_f(multiarg_callback);
    Wrapper_test_multiarg_void_callback(diplomat_callback_wrapper__multiarg_callback);

    DiplomatCallback_Wrapper_test_mod_array_g diplomat_callback_wrapper__mod_array_cb =
        C_create_DiplomatCallback_Wrapper_test_mod_array_g(mod_array_cb);
    Wrapper_test_mod_array(diplomat_callback_wrapper__mod_array_cb);

    DiplomatCallback_Wrapper_test_no_args_h diplomat_callback_wrapper__no_arg_cb =
        C_create_DiplomatCallback_Wrapper_test_no_args_h(no_arg_cb);
    res = Wrapper_test_no_args(diplomat_callback_wrapper__no_arg_cb);
    printf("Got %d back from Rust\n", res);

    DiplomatCallback_Wrapper_test_cb_with_struct_f diplomat_callback_wrapper__deal_with_struct =
        C_create_DiplomatCallback_Wrapper_test_cb_with_struct_f(deal_with_struct);
    res = Wrapper_test_cb_with_struct(diplomat_callback_wrapper__deal_with_struct);
    printf("Got %d back from summing the struct fields\n", res);

    DiplomatCallback_Wrapper_test_multiple_cb_args_g diplomat_callback_wrapper__callback_g = 
        C_create_DiplomatCallback_Wrapper_test_multiple_cb_args_g(callback);
    DiplomatCallback_Wrapper_test_multiple_cb_args_f diplomat_callback_wrapper__get_int_with_no_args = 
        C_create_DiplomatCallback_Wrapper_test_multiple_cb_args_f(get_int_with_no_args);
    res = Wrapper_test_multiple_cb_args(diplomat_callback_wrapper__get_int_with_no_args, diplomat_callback_wrapper__callback_g);
    printf("And now the result of combining the results of 2 callbacks: %d\n", res);
}