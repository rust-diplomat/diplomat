#ifndef ICU4XDataProvider_H
#define ICU4XDataProvider_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XDataProvider ICU4XDataProvider;
#include "diplomat_result_void_void.h"

ICU4XDataProvider* ICU4XDataProvider_new_static();

diplomat_result_void_void ICU4XDataProvider_returns_result();
void ICU4XDataProvider_destroy(ICU4XDataProvider* self);

#ifdef __cplusplus
}
#endif
#endif
