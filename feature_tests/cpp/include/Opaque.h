#ifndef Opaque_H
#define Opaque_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "Opaque_type.h"
#include "MyStruct_type.h"
#include "ImportedStruct_type.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

Opaque* Opaque_new();

void Opaque_assert_struct(const Opaque* self, MyStruct s);

size_t Opaque_returns_usize();

ImportedStruct Opaque_returns_imported();
void Opaque_destroy(Opaque* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // Opaque_H
