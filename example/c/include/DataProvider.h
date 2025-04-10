#ifndef DataProvider_H
#define DataProvider_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "DataProvider.d.h"






DataProvider* icu4x_DataProvider_new_static_mv1(void);

typedef struct icu4x_DataProvider_returns_result_mv1_result { bool is_ok;} icu4x_DataProvider_returns_result_mv1_result;
icu4x_DataProvider_returns_result_mv1_result icu4x_DataProvider_returns_result_mv1(void);


void icu4x_DataProvider_destroy_mv1(DataProvider* self);





#endif // DataProvider_H
