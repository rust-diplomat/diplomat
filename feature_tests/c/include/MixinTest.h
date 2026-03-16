#ifndef MixinTest_H
#define MixinTest_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "MixinTest.d.h"






void namespace_MixinTest_hello(DiplomatWrite* write);

void namespace_MixinTest_destroy(MixinTest* self);





#endif // MixinTest_H
