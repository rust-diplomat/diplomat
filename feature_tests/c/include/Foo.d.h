#ifndef Foo_D_H
#define Foo_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct Foo Foo;


typedef struct DiplomatFooView {
  const Foo** data;
  size_t len;
} DiplomatFooView;



#endif // Foo_D_H
