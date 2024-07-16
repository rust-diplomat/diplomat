#ifndef icu4x_DataProvider_H
#define icu4x_DataProvider_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "icu4x_DataProvider.d.h"






icu4x_DataProvider* icu4x_DataProvider_new_static() {
    icu4x_DataProvider* icu4x_DataProvider_new_static_mv1();
    return icu4x_DataProvider_new_static_mv1();
}

typedef struct icu4x_DataProvider_returns_result_result { bool is_ok;} icu4x_DataProvider_returns_result_result;
icu4x_DataProvider_returns_result_result icu4x_DataProvider_returns_result() {
    icu4x_DataProvider_returns_result_result icu4x_DataProvider_returns_result_mv1();
    return icu4x_DataProvider_returns_result_mv1();
}


void icu4x_DataProvider_destroy(icu4x_DataProvider* self) {
    void icu4x_DataProvider_destroy_mv1(icu4x_DataProvider* self);
    icu4x_DataProvider_destroy_mv1(self);
}





#endif // icu4x_DataProvider_H
