#ifndef OpaqueMutexedString_H
#define OpaqueMutexedString_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct OpaqueMutexedString OpaqueMutexedString;
#ifdef __cplusplus
} // namespace capi
#endif
#include "Utf16Wrap.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

OpaqueMutexedString* OpaqueMutexedString_from_usize(size_t number);

void OpaqueMutexedString_change(const OpaqueMutexedString* self, size_t number);

size_t OpaqueMutexedString_get_len_and_add(const OpaqueMutexedString* self, size_t other);

DiplomatStringView OpaqueMutexedString_dummy_str(const OpaqueMutexedString* self);

Utf16Wrap* OpaqueMutexedString_wrapper(const OpaqueMutexedString* self);
void OpaqueMutexedString_destroy(OpaqueMutexedString* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
