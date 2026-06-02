#ifndef AttrOpaque1_D_H
#define AttrOpaque1_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct AttrOpaque1 AttrOpaque1;


typedef struct DiplomatAttrOpaque1View {
  const AttrOpaque1** data;
  size_t len;
} DiplomatAttrOpaque1View;



#endif // AttrOpaque1_D_H
