#ifndef OptionStruct_type_H
#define OptionStruct_type_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

typedef struct OptionOpaque OptionOpaque;
typedef struct OptionOpaqueChar OptionOpaqueChar;
#ifdef __cplusplus
namespace capi {
#endif // __cplusplus

typedef struct OptionStruct {
    OptionOpaque* a;
    OptionOpaqueChar* b;
    uint32_t c;
    OptionOpaque* d;
} OptionStruct;
#ifdef __cplusplus
} // namespace capi
#endif // __cplusplus
#endif // OptionStruct_type_H
