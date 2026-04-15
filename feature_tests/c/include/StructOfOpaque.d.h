#ifndef StructOfOpaque_D_H
#define StructOfOpaque_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Opaque.d.h"
#include "OpaqueMut.d.h"




typedef struct StructOfOpaque {
  const Opaque* i;
  OpaqueMut* j;
} StructOfOpaque;

typedef struct StructOfOpaque_option {union { StructOfOpaque ok; }; bool is_ok; } StructOfOpaque_option;



#endif // StructOfOpaque_D_H
