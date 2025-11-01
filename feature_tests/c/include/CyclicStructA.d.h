#ifndef CyclicStructA_D_H
#define CyclicStructA_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CyclicStructB.d.h"




typedef struct CyclicStructA {
  CyclicStructB a;
} CyclicStructA;

typedef struct CyclicStructA_option {union { CyclicStructA ok; }; bool is_ok; } CyclicStructA_option;
typedef struct DiplomatCyclicStructAView {
  const CyclicStructA* data;
  size_t len;
} DiplomatCyclicStructAView;

typedef struct DiplomatCyclicStructAViewMut {
  CyclicStructA* data;
  size_t len;
} DiplomatCyclicStructAViewMut;




#endif // CyclicStructA_D_H
