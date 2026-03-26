#ifndef Opaque_D_H
#define Opaque_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct Opaque Opaque;


typedef struct DiplomatOpaqueView {
  const Opaque** data;
  size_t len;
} DiplomatOpaqueView;



#endif // Opaque_D_H
