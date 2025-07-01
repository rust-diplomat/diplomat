#ifndef CallbackWrapper_D_H
#define CallbackWrapper_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct CallbackWrapper {
  bool cant_be_empty;
} CallbackWrapper;

typedef struct CallbackWrapper_option {union { CallbackWrapper ok; }; bool is_ok; } CallbackWrapper_option;



#endif // CallbackWrapper_D_H
