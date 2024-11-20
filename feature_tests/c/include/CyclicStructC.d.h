#ifndef CyclicStructC_D_H
#define CyclicStructC_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CyclicStructA.d.h"




typedef struct CyclicStructC {
  CyclicStructA a;
} CyclicStructC;

typedef struct CyclicStructC_option {union { CyclicStructC ok; }; bool is_ok; } CyclicStructC_option;



#endif // CyclicStructC_D_H
