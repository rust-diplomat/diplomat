#ifndef MyOpaqueEnum_H
#define MyOpaqueEnum_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "MyOpaqueEnum.d.h"






MyOpaqueEnum* MyOpaqueEnum_new(void);

void MyOpaqueEnum_to_string(const MyOpaqueEnum* self, DiplomatWrite* write);

void MyOpaqueEnum_destroy(MyOpaqueEnum* self);





#endif // MyOpaqueEnum_H
