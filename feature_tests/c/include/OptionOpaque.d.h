#ifndef OptionOpaque_D_H
#define OptionOpaque_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct OptionOpaque OptionOpaque;


typedef struct DiplomatOptionOpaqueView {
  const OptionOpaque** data;
  size_t len;
} DiplomatOptionOpaqueView;



#endif // OptionOpaque_D_H
