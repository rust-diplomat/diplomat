#ifndef ImportedStruct_type_H
#define ImportedStruct_type_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "UnimportedEnum_type.h"
#ifdef __cplusplus
namespace capi {
#endif // __cplusplus

typedef struct ImportedStruct {
    UnimportedEnum foo;
    uint8_t count;
} ImportedStruct;
#ifdef __cplusplus
} // namespace capi
#endif // __cplusplus
#endif // ImportedStruct_type_H
