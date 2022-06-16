#ifndef Alpha_H
#define Alpha_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Alpha {
    uint32_t x;
    uint32_t y;
} Alpha;

void Alpha_destroy(Alpha* self);

#ifdef __cplusplus
}
#endif
#endif
