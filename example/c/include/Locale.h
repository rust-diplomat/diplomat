#ifndef Locale_H
#define Locale_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct Locale Locale;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

Locale* icu4x_Locale_new_mv1(const char* name_data, size_t name_len);
void icu4x_Locale_destroy_mv1(Locale* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
