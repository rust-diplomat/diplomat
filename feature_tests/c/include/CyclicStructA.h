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






#endif // CyclicStructA_H
