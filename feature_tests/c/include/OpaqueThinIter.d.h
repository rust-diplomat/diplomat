#ifndef OpaqueThinIter_D_H
#define OpaqueThinIter_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct OpaqueThinIter OpaqueThinIter;


typedef struct DiplomatOpaqueThinIterView {
  const OpaqueThinIter** data;
  size_t len;
} DiplomatOpaqueThinIterView;



#endif // OpaqueThinIter_D_H
