#ifndef Utf16Wrap_H
#define Utf16Wrap_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct Utf16Wrap Utf16Wrap;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

Utf16Wrap* Utf16Wrap_from_utf16(const char16_t* input_data, size_t input_len);

void Utf16Wrap_get_debug_str(const Utf16Wrap* self, DiplomatWrite* write);

DiplomatU16View Utf16Wrap_borrow_cont(const Utf16Wrap* self);

DiplomatU16View Utf16Wrap_owned(const Utf16Wrap* self);
void Utf16Wrap_destroy(Utf16Wrap* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
