#ifndef DeprecatedOpaque_D_H
#define DeprecatedOpaque_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct DeprecatedOpaque DeprecatedOpaque;


typedef struct DiplomatDeprecatedOpaqueView {
  const DeprecatedOpaque** data;
  size_t len;
} DiplomatDeprecatedOpaqueView;



#endif // DeprecatedOpaque_D_H
