#ifndef Bar_H
#define Bar_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct Bar Bar;
#ifdef __cplusplus
} // namespace capi
#endif
#include "Foo.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif
void Bar_destroy(Bar* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
