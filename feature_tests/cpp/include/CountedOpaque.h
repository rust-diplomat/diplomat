#ifndef CountedOpaque_H
#define CountedOpaque_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct CountedOpaque CountedOpaque;
#include "Counter.h"

CountedOpaque* CountedOpaque_new(const Counter* counter);
void CountedOpaque_destroy(CountedOpaque* self);

#ifdef __cplusplus
}
#endif
#endif
