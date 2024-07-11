#ifndef ICU4XDataProvider_H
#define ICU4XDataProvider_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "ICU4XDataProvider.d.h"






ICU4XDataProvider* icu4x_ICU4XDataProvider_new_static_mv1();

typedef struct icu4x_ICU4XDataProvider_returns_result_mv1_result { bool is_ok;} icu4x_ICU4XDataProvider_returns_result_mv1_result;
icu4x_ICU4XDataProvider_returns_result_mv1_result icu4x_ICU4XDataProvider_returns_result_mv1();


void icu4x_ICU4XDataProvider_mv1_destroy(ICU4XDataProvider* self);





#endif // ICU4XDataProvider_H
