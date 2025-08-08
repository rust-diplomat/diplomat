#ifndef OpaqueThin_H
#define OpaqueThin_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "OpaqueThin.d.h"






int32_t OpaqueThin_a(const OpaqueThin* self);

float OpaqueThin_b(const OpaqueThin* self);

void OpaqueThin_c(const OpaqueThin* self, DiplomatWrite* write);

void OpaqueThin_destroy(OpaqueThin* self);





#endif // OpaqueThin_H
