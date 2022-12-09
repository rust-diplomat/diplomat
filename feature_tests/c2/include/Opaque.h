#ifndef Opaque_H
#define Opaque_H


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ImportedStruct.d.h"
#include "MyStruct.d.h"
#include "Opaque.d.h"


#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus

Opaque* Opaque_new();
void Opaque_assert_struct(const Opaque* self, MyStruct s);
size_t Opaque_returns_usize();
ImportedStruct Opaque_returns_imported();
void Opaque_destroy(Opaque* self);


#ifdef __cplusplus
} // namespace capi
} // extern "C"
#endif // __cplusplus

#endif // Opaque_H
