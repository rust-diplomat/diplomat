#ifndef result_ErrorEnum_box_ResultOpaque_H
#define result_ErrorEnum_box_ResultOpaque_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "ErrorEnum.h"
typedef struct ResultOpaque ResultOpaque;
typedef struct feature_tests_result_ffi_result_ErrorEnum_box_ResultOpaque {
    union {
        ErrorEnum ok;
        ResultOpaque* err;
    };
    bool is_ok;
} feature_tests_result_ffi_result_ErrorEnum_box_ResultOpaque;
#ifdef __cplusplus
}
#endif
#endif
