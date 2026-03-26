#ifndef Nested_D_H
#define Nested_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct Nested Nested;


typedef struct DiplomatNestedView {
  const Nested** data;
  size_t len;
} DiplomatNestedView;



#endif // Nested_D_H
