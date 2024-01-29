#ifndef BorrowedFieldsWithBounds_H
#define BorrowedFieldsWithBounds_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct BorrowedFieldsWithBounds {
    DiplomatU16View field_a;
    DiplomatStringView field_b;
    DiplomatStringView field_c;
} BorrowedFieldsWithBounds;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void BorrowedFieldsWithBounds_destroy(BorrowedFieldsWithBounds* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
