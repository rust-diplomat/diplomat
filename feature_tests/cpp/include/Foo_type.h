#ifndef Foo_type_H
#define Foo_type_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif // __cplusplus

typedef struct Foo Foo;
#ifdef __cplusplus
} // namespace capi
#endif // __cplusplus
#endif // Foo_type_H
