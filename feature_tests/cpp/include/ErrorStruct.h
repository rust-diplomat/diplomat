#ifndef ErrorStruct_H
#define ErrorStruct_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ErrorStruct_type.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void ErrorStruct_destroy(ErrorStruct* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ErrorStruct_H
