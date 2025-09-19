#ifndef PrimitiveStruct_H
#define PrimitiveStruct_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "PrimitiveStruct.d.h"






void PrimitiveStruct_mutable_slice(DiplomatPrimitiveStructViewMut a);

void PrimitiveStruct_mutable_ref(PrimitiveStruct* self, PrimitiveStruct* a);





#endif // PrimitiveStruct_H
