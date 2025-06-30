#ifndef BigStructWithStuff_D_H
#define BigStructWithStuff_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ScalarPairWithPadding.d.h"




typedef struct BigStructWithStuff {
  uint8_t first;
  uint16_t second;
  uint16_t third;
  ScalarPairWithPadding fourth;
  uint8_t fifth;
} BigStructWithStuff;

typedef struct BigStructWithStuff_option {union { BigStructWithStuff ok; }; bool is_ok; } BigStructWithStuff_option;



#endif // BigStructWithStuff_D_H
