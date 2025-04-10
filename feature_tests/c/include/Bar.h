#ifndef Bar_H
#define Bar_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Foo.d.h"

#include "Bar.d.h"






const Foo* Bar_foo(const Bar* self);

void Bar_destroy(Bar* self);





#endif // Bar_H
