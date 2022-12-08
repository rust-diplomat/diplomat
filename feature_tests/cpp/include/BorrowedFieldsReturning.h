#ifndef BorrowedFieldsReturning_H
#define BorrowedFieldsReturning_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct BorrowedFieldsReturning {
    DiplomatU8View bytes;
} BorrowedFieldsReturning;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void BorrowedFieldsReturning_destroy(BorrowedFieldsReturning* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
