#ifndef diplomat_result_box_ICU4XFixedDecimalFormatter_void_H
#define diplomat_result_box_ICU4XFixedDecimalFormatter_void_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

typedef struct ICU4XFixedDecimalFormatter ICU4XFixedDecimalFormatter;
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif
typedef struct diplomat_result_box_ICU4XFixedDecimalFormatter_void {
    union {
        ICU4XFixedDecimalFormatter* ok;
    };
    bool is_ok;
} diplomat_result_box_ICU4XFixedDecimalFormatter_void;
#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
