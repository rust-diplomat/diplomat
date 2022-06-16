#ifndef Beta_H
#define Beta_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "Alpha.h"

typedef struct Beta {
    Alpha alpha_field;
} Beta;

Beta Beta_new(uint32_t x, uint32_t y);
void Beta_destroy(Beta* self);

#ifdef __cplusplus
}
#endif
#endif
