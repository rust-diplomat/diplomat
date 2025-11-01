#ifndef StructWithAttrs_D_H
#define StructWithAttrs_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct StructWithAttrs {
  bool a;
  uint32_t b;
} StructWithAttrs;

typedef struct StructWithAttrs_option {union { StructWithAttrs ok; }; bool is_ok; } StructWithAttrs_option;
typedef struct DiplomatStructWithAttrsView {
  const StructWithAttrs* data;
  size_t len;
} DiplomatStructWithAttrsView;

typedef struct DiplomatStructWithAttrsViewMut {
  StructWithAttrs* data;
  size_t len;
} DiplomatStructWithAttrsViewMut;




#endif // StructWithAttrs_D_H
