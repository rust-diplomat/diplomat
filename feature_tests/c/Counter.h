#ifndef Counter_H
#define Counter_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Counter Counter;

Counter* Counter_new();

size_t Counter_count(const Counter* self);
void Counter_destroy(Counter* self);

#ifdef __cplusplus
}
#endif
#endif
