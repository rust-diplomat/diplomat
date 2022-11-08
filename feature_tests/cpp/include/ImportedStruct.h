#ifndef ImportedStruct_H
#define ImportedStruct_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ImportedStruct_type.h"
#include "UnimportedEnum_type.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void ImportedStruct_destroy(ImportedStruct* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ImportedStruct_H
