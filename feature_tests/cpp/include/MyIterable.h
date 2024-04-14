#ifndef MyIterable_H
#define MyIterable_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct MyIterable MyIterable;
#ifdef __cplusplus
} // namespace capi
#endif
#include "MyIterator.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

MyIterable* namespace_MyIterable_new(const uint8_t* x_data, size_t x_len);

MyIterator* namespace_MyIterable_iter(const MyIterable* self);
void namespace_MyIterable_destroy(MyIterable* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
