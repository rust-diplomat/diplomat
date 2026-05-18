#ifndef MutableCallbackHolder_D_H
#define MutableCallbackHolder_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct MutableCallbackHolder MutableCallbackHolder;


typedef struct DiplomatMutableCallbackHolderView {
  const MutableCallbackHolder** data;
  size_t len;
} DiplomatMutableCallbackHolderView;



#endif // MutableCallbackHolder_D_H
