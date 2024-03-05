#ifndef NestedBorrowedFields_H
#define NestedBorrowedFields_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BorrowedFields.h"
#include "BorrowedFieldsWithBounds.h"
#ifdef __cplusplus
namespace capi {
#endif

typedef struct NestedBorrowedFields {
    BorrowedFields fields;
    BorrowedFieldsWithBounds bounds;
    BorrowedFieldsWithBounds bounds2;
} NestedBorrowedFields;
#ifdef __cplusplus
} // namespace capi
#endif
#include "BorrowedFields.h"
#include "BorrowedFieldsWithBounds.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void NestedBorrowedFields_destroy(NestedBorrowedFields* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
