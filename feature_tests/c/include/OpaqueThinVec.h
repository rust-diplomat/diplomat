#ifndef OpaqueThinVec_H
#define OpaqueThinVec_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "OpaqueThin.d.h"
#include "OpaqueThinIter.d.h"

#include "OpaqueThinVec.d.h"






OpaqueThinVec* OpaqueThinVec_create(DiplomatI32View a, DiplomatF32View b, DiplomatStringView c);

OpaqueThinIter* OpaqueThinVec_iter(const OpaqueThinVec* self);

size_t OpaqueThinVec_len(const OpaqueThinVec* self);

const OpaqueThin* OpaqueThinVec_get(const OpaqueThinVec* self, size_t idx);

const OpaqueThin* OpaqueThinVec_first(const OpaqueThinVec* self);

void OpaqueThinVec_destroy(OpaqueThinVec* self);





#endif // OpaqueThinVec_H
