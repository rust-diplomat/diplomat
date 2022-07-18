#ifndef Opaque_H
#define Opaque_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct Opaque Opaque;
#ifdef __cplusplus
} // namespace capi
#endif
#include "MyStruct.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

Opaque* Opaque_new();

void Opaque_assert_struct(const Opaque* self, MyStruct s);
void Opaque_destroy(Opaque* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
