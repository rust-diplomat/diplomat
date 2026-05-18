#ifndef Float64Vec_D_H
#define Float64Vec_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct Float64Vec Float64Vec;


typedef struct DiplomatFloat64VecView {
  const Float64Vec** data;
  size_t len;
} DiplomatFloat64VecView;



#endif // Float64Vec_D_H
