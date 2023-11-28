#ifndef BorrowedFields_H
#define BorrowedFields_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct BorrowedFields {
    DiplomatU16View a;
    DiplomatStringView b;
    DiplomatStringView c;
} BorrowedFields;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void BorrowedFields_destroy(BorrowedFields* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
