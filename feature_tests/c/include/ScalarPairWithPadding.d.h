#ifndef ScalarPairWithPadding_D_H
#define ScalarPairWithPadding_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct ScalarPairWithPadding {
  uint8_t first;
  uint32_t second;
} ScalarPairWithPadding;

typedef struct ScalarPairWithPadding_option {union { ScalarPairWithPadding ok; }; bool is_ok; } ScalarPairWithPadding_option;
typedef struct DiplomatScalarPairWithPaddingView {
  const ScalarPairWithPadding* data;
  size_t len;
} DiplomatScalarPairWithPaddingView;

typedef struct DiplomatScalarPairWithPaddingViewMut {
  ScalarPairWithPadding* data;
  size_t len;
} DiplomatScalarPairWithPaddingViewMut;




#endif // ScalarPairWithPadding_D_H
