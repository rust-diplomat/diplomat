#ifndef diplomat_result_void_box_ResultOpaque_H
#define diplomat_result_void_box_ResultOpaque_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ResultOpaque.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif
typedef struct diplomat_result_void_box_ResultOpaque {
    union {
        ResultOpaque* err;
    };
    bool is_ok;
} diplomat_result_void_box_ResultOpaque;
#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
