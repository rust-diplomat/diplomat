#ifndef PrimitiveStructVec_H
#define PrimitiveStructVec_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "PrimitiveStruct.d.h"

#include "PrimitiveStructVec.d.h"






PrimitiveStructVec* PrimitiveStructVec_new(void);

void PrimitiveStructVec_push(PrimitiveStructVec* self, PrimitiveStruct value);

size_t PrimitiveStructVec_len(const PrimitiveStructVec* self);

DiplomatPrimitiveStructView PrimitiveStructVec_as_slice(const PrimitiveStructVec* self);

void PrimitiveStructVec_destroy(PrimitiveStructVec* self);





#endif // PrimitiveStructVec_H
