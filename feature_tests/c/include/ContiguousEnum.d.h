#ifndef ContiguousEnum_D_H
#define ContiguousEnum_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum ContiguousEnum {
  ContiguousEnum_C = 0,
  ContiguousEnum_D = 1,
  ContiguousEnum_E = 2,
  ContiguousEnum_F = 3,
} ContiguousEnum;

typedef struct ContiguousEnum_option {union { ContiguousEnum ok; }; bool is_ok; } ContiguousEnum_option;
typedef struct DiplomatContiguousEnumView {
  const ContiguousEnum* data;
  size_t len;
} DiplomatContiguousEnumView;

typedef struct DiplomatContiguousEnumViewMut {
  ContiguousEnum* data;
  size_t len;
} DiplomatContiguousEnumViewMut;




#endif // ContiguousEnum_D_H
