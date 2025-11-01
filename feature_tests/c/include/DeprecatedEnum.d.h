#ifndef DeprecatedEnum_D_H
#define DeprecatedEnum_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum DeprecatedEnum {
  DeprecatedEnum_A = 0,
} DeprecatedEnum;

typedef struct DeprecatedEnum_option {union { DeprecatedEnum ok; }; bool is_ok; } DeprecatedEnum_option;



#endif // DeprecatedEnum_D_H
