#ifndef AttrEnum_D_H
#define AttrEnum_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




typedef enum AttrEnum {
  AttrEnum_A = 0,
  AttrEnum_B = 1,
  AttrEnum_C = 2,
} AttrEnum;

typedef struct AttrEnum_option {union { AttrEnum ok; }; bool is_ok; } AttrEnum_option;

#endif // AttrEnum_D_H
