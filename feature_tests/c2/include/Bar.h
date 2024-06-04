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

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus



const Foo* Bar_foo(const Bar* self);

void Bar_destroy(Bar* self);



#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // Bar_H
