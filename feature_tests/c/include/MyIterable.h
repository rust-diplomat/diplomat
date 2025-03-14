#ifndef MyIterable_H
#define MyIterable_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "MyIterator.d.h"

#include "MyIterable.d.h"






MyIterable* namespace_MyIterable_new(DiplomatU8View x);

MyIterator* namespace_MyIterable_iter(const MyIterable* self);


void namespace_MyIterable_destroy(MyIterable* self);





#endif // MyIterable_H
