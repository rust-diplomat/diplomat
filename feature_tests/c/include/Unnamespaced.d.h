#ifndef Unnamespaced_D_H
#define Unnamespaced_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct Unnamespaced Unnamespaced;


typedef struct DiplomatUnnamespacedView {
  const Unnamespaced** data;
  size_t len;
} DiplomatUnnamespacedView;



#endif // Unnamespaced_D_H
