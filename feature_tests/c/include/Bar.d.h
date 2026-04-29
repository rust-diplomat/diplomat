#ifndef Bar_D_H
#define Bar_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct Bar Bar;


typedef struct DiplomatBarView {
  const Bar** data;
  size_t len;
} DiplomatBarView;



#endif // Bar_D_H
