#ifndef MixinTest_D_H
#define MixinTest_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct MixinTest MixinTest;


typedef struct DiplomatMixinTestView {
  const MixinTest** data;
  size_t len;
} DiplomatMixinTestView;



#endif // MixinTest_D_H
