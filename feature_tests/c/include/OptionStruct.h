#ifndef OptionStruct_H
#define OptionStruct_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "OptionOpaque.h"
#include "OptionOpaqueChar.h"
#ifdef __cplusplus
namespace capi {
#endif

typedef struct OptionStruct {
    OptionOpaque* a;
    OptionOpaqueChar* b;
    uint32_t c;
    OptionOpaque* d;
} OptionStruct;
#ifdef __cplusplus
} // namespace capi
#endif
#include "OptionOpaque.h"
#include "OptionOpaqueChar.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void OptionStruct_destroy(OptionStruct* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
