#ifndef result_box_ResultOpaque_void_H
#define result_box_ResultOpaque_void_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif
typedef struct ResultOpaque ResultOpaque;
typedef struct result_ffi_result_box_ResultOpaque_void {
    union {
        ResultOpaque* ok;
    };
    bool is_ok;
} result_ffi_result_box_ResultOpaque_void;
#ifdef __cplusplus
}
#endif
#endif
