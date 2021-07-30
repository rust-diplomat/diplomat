#ifndef data_provider_ffi_ICU4XDataProvider_H
#define data_provider_ffi_ICU4XDataProvider_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif


typedef struct ICU4XDataProvider ICU4XDataProvider;

ICU4XDataProvider* ICU4XDataProvider_new_static();
void ICU4XDataProvider_destroy(ICU4XDataProvider* self);
#ifdef __cplusplus
}
#endif
#endif
