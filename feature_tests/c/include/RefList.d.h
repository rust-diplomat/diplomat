#ifndef RefList_D_H
#define RefList_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct RefList RefList;


typedef struct DiplomatRefListView {
  const RefList** data;
  size_t len;
} DiplomatRefListView;



#endif // RefList_D_H
