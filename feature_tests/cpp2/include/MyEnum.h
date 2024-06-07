#ifndef MyEnum_H
#define MyEnum_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "MyEnum.d.h"

namespace capi {


extern "C" {

int8_t MyEnum_into_value(MyEnum self);

MyEnum MyEnum_get_a();


} // extern "C"

} // namespace capi

#endif // MyEnum_H
