#ifndef DataProvider_H
#define DataProvider_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "DataProvider.d.h"






DataProvider* DataProvider_new_static() {
    DataProvider* icu4x_DataProvider_new_static_mv1();
    return icu4x_DataProvider_new_static_mv1();
}

typedef struct DataProvider_returns_result_result { bool is_ok;} DataProvider_returns_result_result;
DataProvider_returns_result_result DataProvider_returns_result() {
    DataProvider_returns_result_result icu4x_DataProvider_returns_result_mv1();
    return icu4x_DataProvider_returns_result_mv1();
}


void icu4x_DataProvider_destroy_mv1(DataProvider* self);





#endif // DataProvider_H
