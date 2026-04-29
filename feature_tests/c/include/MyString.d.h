#ifndef MyString_D_H
#define MyString_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct MyString MyString;


typedef struct DiplomatMyStringView {
  const MyString** data;
  size_t len;
} DiplomatMyStringView;



#endif // MyString_D_H
