#ifndef ErrorStruct_D_H
#define ErrorStruct_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct ErrorStruct {
  int32_t i;
  int32_t j;
} ErrorStruct;

typedef struct ErrorStruct_option {union { ErrorStruct ok; }; bool is_ok; } ErrorStruct_option;


// TODO: Need to add Mut types.
typedef struct DiplomatErrorStructView {
  const ErrorStruct* data;
  size_t len;
} DiplomatErrorStructView;




#endif // ErrorStruct_D_H
