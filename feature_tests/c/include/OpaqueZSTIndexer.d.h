#ifndef OpaqueZSTIndexer_D_H
#define OpaqueZSTIndexer_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct OpaqueZSTIndexer OpaqueZSTIndexer;


typedef struct DiplomatOpaqueZSTIndexerView {
  const OpaqueZSTIndexer** data;
  size_t len;
} DiplomatOpaqueZSTIndexerView;



#endif // OpaqueZSTIndexer_D_H
