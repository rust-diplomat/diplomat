#ifndef TraitTestingStruct_D_H
#define TraitTestingStruct_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct TraitTestingStruct {
  int32_t x;
  int32_t y;
} TraitTestingStruct;

typedef struct TraitTestingStruct_option {union { TraitTestingStruct ok; }; bool is_ok; } TraitTestingStruct_option;


// TODO: Need to add Mut types.
typedef struct DiplomatTraitTestingStructView {
  const TraitTestingStruct* data;
  size_t len;
} DiplomatTraitTestingStructView;




#endif // TraitTestingStruct_D_H
