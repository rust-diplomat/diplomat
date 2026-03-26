#ifndef FixedDecimalFormatter_D_H
#define FixedDecimalFormatter_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct FixedDecimalFormatter FixedDecimalFormatter;


typedef struct DiplomatFixedDecimalFormatterView {
  const FixedDecimalFormatter** data;
  size_t len;
} DiplomatFixedDecimalFormatterView;



#endif // FixedDecimalFormatter_D_H
