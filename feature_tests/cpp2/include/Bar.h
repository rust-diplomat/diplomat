#ifndef Bar_H
#define Bar_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Foo.d.h"
#include "Foo.h"

#include "Bar.d.h"

namespace capi {


extern "C" {

const Foo* Bar_foo(const Bar* self);


void Bar_destroy(Bar* self);

} // extern "C"

} // namespace capi

#endif // Bar_H
