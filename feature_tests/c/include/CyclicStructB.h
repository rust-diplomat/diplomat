#ifndef CyclicStructB_H
#define CyclicStructB_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CyclicStructA.d.h"

#include "CyclicStructB.d.h"






CyclicStructA CyclicStructB_get_a(void);

typedef struct CyclicStructB_get_a_option_result {union {CyclicStructA ok; }; bool is_ok;} CyclicStructB_get_a_option_result;
CyclicStructB_get_a_option_result CyclicStructB_get_a_option(void);





#endif // CyclicStructB_H
