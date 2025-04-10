#ifndef OpaqueMutexedString_H
#define OpaqueMutexedString_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Utf16Wrap.d.h"

#include "OpaqueMutexedString.d.h"






OpaqueMutexedString* OpaqueMutexedString_from_usize(size_t number);

void OpaqueMutexedString_change(const OpaqueMutexedString* self, size_t number);

const OpaqueMutexedString* OpaqueMutexedString_borrow(const OpaqueMutexedString* self);

const OpaqueMutexedString* OpaqueMutexedString_borrow_other(const OpaqueMutexedString* other);

const OpaqueMutexedString* OpaqueMutexedString_borrow_self_or_other(const OpaqueMutexedString* self, const OpaqueMutexedString* other);

size_t OpaqueMutexedString_get_len_and_add(const OpaqueMutexedString* self, size_t other);

DiplomatStringView OpaqueMutexedString_dummy_str(const OpaqueMutexedString* self);

Utf16Wrap* OpaqueMutexedString_wrapper(const OpaqueMutexedString* self);

uint16_t OpaqueMutexedString_to_unsigned_from_unsigned(const OpaqueMutexedString* self, uint16_t input);

void OpaqueMutexedString_destroy(OpaqueMutexedString* self);





#endif // OpaqueMutexedString_H
