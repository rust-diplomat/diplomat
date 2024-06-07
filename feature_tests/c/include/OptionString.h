#ifndef OptionString_H
#define OptionString_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct OptionString OptionString;
#ifdef __cplusplus
} // namespace capi
#endif
#include "diplomat_result_void_void.h"
#include "diplomat_result_str_ref8_void.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

OptionString* OptionString_new(const char* diplomat_str_data, size_t diplomat_str_len);

diplomat_result_void_void OptionString_write(const OptionString* self, DiplomatWrite* write);

diplomat_result_str_ref8_void OptionString_borrow(const OptionString* self);
void OptionString_destroy(OptionString* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
