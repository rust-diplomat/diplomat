#ifndef OptionEnum_D_H
#define OptionEnum_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum OptionEnum {
  OptionEnum_Foo = 0,
  OptionEnum_Bar = 1,
  OptionEnum_Baz = 2,
} OptionEnum;

typedef struct OptionEnum_option {union { OptionEnum ok; }; bool is_ok; } OptionEnum_option;



#endif // OptionEnum_D_H
