#ifndef Bar_H
#define Bar_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Bar Bar;

void Bar_destroy(Bar* self);

#ifdef __cplusplus
}
#endif
#endif
