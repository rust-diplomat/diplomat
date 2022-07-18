#ifndef ICU4XLocale_H
#define ICU4XLocale_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XLocale ICU4XLocale;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

ICU4XLocale* ICU4XLocale_new(const char* name_data, size_t name_len);

ICU4XLocale* ICU4XLocale_new_from_bytes(const uint8_t* bytes_data, size_t bytes_len);
void ICU4XLocale_destroy(ICU4XLocale* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
