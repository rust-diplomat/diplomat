#ifndef CyclicStructA_H
#define CyclicStructA_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CyclicStructB.d.h"

#include "CyclicStructA.d.h"






CyclicStructB CyclicStructA_get_b(void);

void CyclicStructA_cyclic_out(CyclicStructA self, DiplomatWrite* write);

void CyclicStructA_double_cyclic_out(CyclicStructA self, CyclicStructA cyclic_struct_a, DiplomatWrite* write);

void CyclicStructA_getter_out(CyclicStructA self, DiplomatWrite* write);






#endif // CyclicStructA_H
