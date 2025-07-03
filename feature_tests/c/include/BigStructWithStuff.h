#ifndef BigStructWithStuff_H
#define BigStructWithStuff_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "BigStructWithStuff.d.h"






void BigStructWithStuff_assert_value(BigStructWithStuff self, uint16_t extra_val);

void BigStructWithStuff_assert_slice(DiplomatBigStructWithStuffView slice, uint16_t second_value);





#endif // BigStructWithStuff_H
