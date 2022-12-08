#ifndef Foo_H
#define Foo_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct Foo Foo;
#ifdef __cplusplus
} // namespace capi
#endif
#include "Bar.h"
#include "BorrowedFields.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

Foo* Foo_new(const char* x_data, size_t x_len);

Bar* Foo_get_bar(const Foo* self);

Foo* Foo_new_static(const char* x_data, size_t x_len);

Foo* Foo_extract_from_fields(BorrowedFields fields);
void Foo_destroy(Foo* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
