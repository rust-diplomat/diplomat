#ifndef MyOpaqueEnum_D_H
#define MyOpaqueEnum_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct MyOpaqueEnum MyOpaqueEnum;


typedef struct DiplomatMyOpaqueEnumView {
  const MyOpaqueEnum** data;
  size_t len;
} DiplomatMyOpaqueEnumView;



#endif // MyOpaqueEnum_D_H
