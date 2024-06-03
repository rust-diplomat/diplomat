#ifndef MyStruct_H
#define MyStruct_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "MyStruct.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


MyStruct MyStruct_new();

uint8_t MyStruct_into_a(MyStruct self);



#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // MyStruct_H
