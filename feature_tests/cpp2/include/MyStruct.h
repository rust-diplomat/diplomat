#ifndef MyStruct_H
#define MyStruct_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "MyStruct.d.h"

namespace capi {


extern "C" {

MyStruct MyStruct_new();

uint8_t MyStruct_into_a(MyStruct self);


} // extern "C"

} // namespace capi

#endif // MyStruct_H
