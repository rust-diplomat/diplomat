#ifndef OpaqueMutexedString_D_H
#define OpaqueMutexedString_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct OpaqueMutexedString OpaqueMutexedString;


typedef struct DiplomatOpaqueMutexedStringView {
  const OpaqueMutexedString** data;
  size_t len;
} DiplomatOpaqueMutexedStringView;



#endif // OpaqueMutexedString_D_H
