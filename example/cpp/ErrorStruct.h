#ifndef ErrorStruct_H
#define ErrorStruct_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ErrorStruct {
    int32_t i;
} ErrorStruct;

void ErrorStruct_destroy(ErrorStruct* self);

#ifdef __cplusplus
}
#endif
#endif
