#ifndef ICU4XLocale_H
#define ICU4XLocale_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "ICU4XLocale.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


ICU4XLocale* ICU4XLocale_new(const char* name_data, size_t name_len);

void ICU4XLocale_destroy(ICU4XLocale* self);



#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XLocale_H
