#ifndef MyZst_D_H
#define MyZst_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"










// TODO: Need to add Mut types.
typedef struct DiplomatMyZstView {
  const MyZst* data;
  size_t len;
} DiplomatMyZstView;




#endif // MyZst_D_H
