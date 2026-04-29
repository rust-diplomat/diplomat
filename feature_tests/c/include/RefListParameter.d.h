#ifndef RefListParameter_D_H
#define RefListParameter_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct RefListParameter RefListParameter;


typedef struct DiplomatRefListParameterView {
  const RefListParameter** data;
  size_t len;
} DiplomatRefListParameterView;



#endif // RefListParameter_D_H
