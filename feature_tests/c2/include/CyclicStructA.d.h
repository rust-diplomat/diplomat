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





#endif // CyclicStructA_D_H
