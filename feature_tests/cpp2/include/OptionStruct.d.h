#ifndef OptionStruct_D_H
#define OptionStruct_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "OptionOpaque.d.h"
#include "OptionOpaqueChar.d.h"

namespace capi {


typedef struct OptionStruct {
  OptionOpaque* a;
  OptionOpaqueChar* b;
  uint32_t c;
  OptionOpaque* d;
} OptionStruct;



} // namespace capi

#endif // OptionStruct_D_H
