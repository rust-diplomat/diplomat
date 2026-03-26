#ifndef OptionOpaqueChar_D_H
#define OptionOpaqueChar_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct OptionOpaqueChar OptionOpaqueChar;


typedef struct DiplomatOptionOpaqueCharView {
  const OptionOpaqueChar** data;
  size_t len;
} DiplomatOptionOpaqueCharView;



#endif // OptionOpaqueChar_D_H
