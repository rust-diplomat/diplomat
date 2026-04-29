#ifndef Utf16Wrap_D_H
#define Utf16Wrap_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct Utf16Wrap Utf16Wrap;


typedef struct DiplomatUtf16WrapView {
  const Utf16Wrap** data;
  size_t len;
} DiplomatUtf16WrapView;



#endif // Utf16Wrap_D_H
