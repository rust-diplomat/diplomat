#ifndef Foo_H
#define Foo_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "Foo_type.h"
#include "Bar_type.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

Foo* Foo_new(const char* x_data, size_t x_len);

Bar* Foo_get_bar(const Foo* self);

Foo* Foo_new_static(const char* x_data, size_t x_len);
void Foo_destroy(Foo* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // Foo_H
