#ifndef ErrorEnum_D_H
#define ErrorEnum_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum ErrorEnum {
  ErrorEnum_Foo = 0,
  ErrorEnum_Bar = 1,
} ErrorEnum;

typedef struct ErrorEnum_option {union { ErrorEnum ok; }; bool is_ok; } ErrorEnum_option;



#endif // ErrorEnum_D_H
