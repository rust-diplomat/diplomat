#ifndef OptionStruct_D_H
#define OptionStruct_D_H


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "OptionOpaque.d.h"
#include "OptionOpaqueChar.d.h"


#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus

typedef struct OptionStruct {
  OptionOpaque* a;
  OptionOpaqueChar* b;
  uint32_t c;
  OptionOpaque* d;
} OptionStruct;





#ifdef __cplusplus
} // namespace capi
} // extern "C"
#endif // __cplusplus

#endif // OptionStruct_D_H
