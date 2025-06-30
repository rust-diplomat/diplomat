#ifndef StructWithSlices_D_H
#define StructWithSlices_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct StructWithSlices {
  DiplomatStringView first;
  DiplomatU16View second;
} StructWithSlices;

typedef struct StructWithSlices_option {union { StructWithSlices ok; }; bool is_ok; } StructWithSlices_option;





#endif // StructWithSlices_D_H
