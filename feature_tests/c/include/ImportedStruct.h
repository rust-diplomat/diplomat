#ifndef ImportedStruct_H
#define ImportedStruct_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "UnimportedEnum.h"
#ifdef __cplusplus
namespace capi {
#endif

typedef struct ImportedStruct {
    UnimportedEnum foo;
    uint8_t count;
} ImportedStruct;
#ifdef __cplusplus
} // namespace capi
#endif
#include "UnimportedEnum.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void ImportedStruct_destroy(ImportedStruct* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
