#ifndef TraitWrapper_D_H
#define TraitWrapper_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct TraitWrapper {
  bool cant_be_empty;
} TraitWrapper;

typedef struct TraitWrapper_option {union { TraitWrapper ok; }; bool is_ok; } TraitWrapper_option;



#endif // TraitWrapper_D_H
