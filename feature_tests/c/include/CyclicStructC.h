#ifndef CyclicStructC_H
#define CyclicStructC_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "CyclicStructC.d.h"






CyclicStructC CyclicStructC_takes_nested_parameters(CyclicStructC c);

void CyclicStructC_cyclic_out(CyclicStructC self, DiplomatWrite* write);





#endif // CyclicStructC_H
