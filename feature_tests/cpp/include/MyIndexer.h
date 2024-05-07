#ifndef MyIndexer_H
#define MyIndexer_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct MyIndexer MyIndexer;
#ifdef __cplusplus
} // namespace capi
#endif
#include "diplomat_result_str_ref8_void.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_str_ref8_void namespace_MyIndexer_get(const MyIndexer* self, size_t i);
void namespace_MyIndexer_destroy(MyIndexer* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
