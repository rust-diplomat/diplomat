#ifndef DefaultEnum_D_H
#define DefaultEnum_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




typedef enum DefaultEnum {
  DefaultEnum_A = 0,
  DefaultEnum_B = 1,
} DefaultEnum;

typedef struct DefaultEnum_option {union { DefaultEnum ok; }; bool is_ok; } DefaultEnum_option;

#endif // DefaultEnum_D_H
