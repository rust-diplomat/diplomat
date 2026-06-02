#ifndef OptionString_D_H
#define OptionString_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct OptionString OptionString;


typedef struct DiplomatOptionStringView {
  const OptionString** data;
  size_t len;
} DiplomatOptionStringView;



#endif // OptionString_D_H
