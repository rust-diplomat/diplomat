#ifndef OpaqueThinIter_H
#define OpaqueThinIter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "OpaqueThin.d.h"

#include "OpaqueThinIter.d.h"






const OpaqueThin* OpaqueThinIter_next(OpaqueThinIter* self);

void OpaqueThinIter_destroy(OpaqueThinIter* self);





#endif // OpaqueThinIter_H
