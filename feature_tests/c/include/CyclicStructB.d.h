#ifndef CyclicStructB_D_H
#define CyclicStructB_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct CyclicStructB {
  uint8_t field;
} CyclicStructB;

typedef struct CyclicStructB_option {union { CyclicStructB ok; }; bool is_ok; } CyclicStructB_option;
typedef struct DiplomatCyclicStructBView {
  const CyclicStructB* data;
  size_t len;
} DiplomatCyclicStructBView;

typedef struct DiplomatCyclicStructBViewMut {
  CyclicStructB* data;
  size_t len;
} DiplomatCyclicStructBViewMut;




#endif // CyclicStructB_D_H
