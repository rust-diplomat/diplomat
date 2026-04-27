#ifndef PrimitiveStructVec_H
#define PrimitiveStructVec_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "PrimitiveStruct.d.h"
#include "StructWithAttrs.d.h"

#include "PrimitiveStructVec.d.h"






PrimitiveStructVec* PrimitiveStructVec_new(void);

void PrimitiveStructVec_push(PrimitiveStructVec* self, PrimitiveStruct value);

size_t PrimitiveStructVec_len(const PrimitiveStructVec* self);

DiplomatPrimitiveStructView PrimitiveStructVec_as_slice(const PrimitiveStructVec* self);

DiplomatPrimitiveStructViewMut PrimitiveStructVec_as_slice_mut(PrimitiveStructVec* self);

typedef struct PrimitiveStructVec_get_result {union {PrimitiveStruct ok; }; bool is_ok;} PrimitiveStructVec_get_result;
PrimitiveStructVec_get_result PrimitiveStructVec_get(const PrimitiveStructVec* self, size_t idx);

void PrimitiveStructVec_take_slice_from_other_namespace(DiplomatStructWithAttrsView _sl);

PrimitiveStructVec* PrimitiveStructVec_take_in_slice(DiplomatPrimitiveStructView a);

void PrimitiveStructVec_destroy(PrimitiveStructVec* self);





#endif // PrimitiveStructVec_H
