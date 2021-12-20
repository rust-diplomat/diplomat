#ifndef result_box_ResultOpaque_ErrorEnum_H
#define result_box_ResultOpaque_ErrorEnum_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif
typedef struct ResultOpaque ResultOpaque;
#include "ErrorEnum.h"
typedef struct result_ffi_result_box_ResultOpaque_ErrorEnum {
    union {
        ResultOpaque* ok;
        ErrorEnum err;
    };
    bool is_ok;
} result_ffi_result_box_ResultOpaque_ErrorEnum;
#ifdef __cplusplus
}
#endif
#endif
