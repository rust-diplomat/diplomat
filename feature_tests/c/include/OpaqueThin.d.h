#ifndef OpaqueThin_D_H
#define OpaqueThin_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct OpaqueThin OpaqueThin;


typedef struct DiplomatOpaqueThinView {
  const OpaqueThin** data;
  size_t len;
} DiplomatOpaqueThinView;



#endif // OpaqueThin_D_H
