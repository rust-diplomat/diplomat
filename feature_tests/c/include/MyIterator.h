#ifndef MyIterator_H
#define MyIterator_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct MyIterator MyIterator;
#ifdef __cplusplus
} // namespace capi
#endif
#include "diplomat_result_uint8_t_void.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_uint8_t_void namespace_MyIterator_next(MyIterator* self);
void namespace_MyIterator_destroy(MyIterator* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
