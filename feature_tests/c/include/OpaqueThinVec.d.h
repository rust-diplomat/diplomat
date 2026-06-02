#ifndef OpaqueThinVec_D_H
#define OpaqueThinVec_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct OpaqueThinVec OpaqueThinVec;


typedef struct DiplomatOpaqueThinVecView {
  const OpaqueThinVec** data;
  size_t len;
} DiplomatOpaqueThinVecView;



#endif // OpaqueThinVec_D_H
