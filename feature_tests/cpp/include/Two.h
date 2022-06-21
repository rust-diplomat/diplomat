#ifndef Two_H
#define Two_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Two Two;

void Two_destroy(Two* self);

#ifdef __cplusplus
}
#endif
#endif
