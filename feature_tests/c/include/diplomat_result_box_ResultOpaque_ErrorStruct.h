#ifndef diplomat_result_box_ResultOpaque_ErrorStruct_H
#define diplomat_result_box_ResultOpaque_ErrorStruct_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif
typedef struct ResultOpaque ResultOpaque;
#include "ErrorStruct.h"
typedef struct diplomat_result_box_ResultOpaque_ErrorStruct {
    union {
        ResultOpaque* ok;
        ErrorStruct err;
    };
    bool is_ok;
} diplomat_result_box_ResultOpaque_ErrorStruct;
#ifdef __cplusplus
}
#endif
#endif
