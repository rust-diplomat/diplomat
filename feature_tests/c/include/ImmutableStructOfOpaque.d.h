#ifndef ImmutableStructOfOpaque_D_H
#define ImmutableStructOfOpaque_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Opaque.d.h"




typedef struct ImmutableStructOfOpaque {
  const Opaque* i;
} ImmutableStructOfOpaque;

typedef struct ImmutableStructOfOpaque_option {union { ImmutableStructOfOpaque ok; }; bool is_ok; } ImmutableStructOfOpaque_option;



#endif // ImmutableStructOfOpaque_D_H
