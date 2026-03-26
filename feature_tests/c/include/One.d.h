#ifndef One_D_H
#define One_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct One One;


typedef struct DiplomatOneView {
  const One** data;
  size_t len;
} DiplomatOneView;



#endif // One_D_H
