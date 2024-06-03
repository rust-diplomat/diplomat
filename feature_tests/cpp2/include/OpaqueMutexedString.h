#ifndef OpaqueMutexedString_H
#define OpaqueMutexedString_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Utf16Wrap.d.h"
#include "Utf16Wrap.h"

#include "OpaqueMutexedString.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus



OpaqueMutexedString* OpaqueMutexedString_from_usize(size_t number);

void OpaqueMutexedString_change(const OpaqueMutexedString* self, size_t number);

const OpaqueMutexedString* OpaqueMutexedString_borrow(const OpaqueMutexedString* self);

const OpaqueMutexedString* OpaqueMutexedString_borrow_other(const OpaqueMutexedString* other);

const OpaqueMutexedString* OpaqueMutexedString_borrow_self_or_other(const OpaqueMutexedString* self, const OpaqueMutexedString* other);

size_t OpaqueMutexedString_get_len_and_add(const OpaqueMutexedString* self, size_t other);

DiplomatStringView OpaqueMutexedString_dummy_str(const OpaqueMutexedString* self);

Utf16Wrap* OpaqueMutexedString_wrapper(const OpaqueMutexedString* self);

void OpaqueMutexedString_destroy(OpaqueMutexedString* self);



#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // OpaqueMutexedString_H
