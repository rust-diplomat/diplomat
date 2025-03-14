#ifndef OpaqueIterable_H
#define OpaqueIterable_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "OpaqueIterator.d.h"

#include "OpaqueIterable.d.h"






OpaqueIterator* namespace_OpaqueIterable_iter(const OpaqueIterable* self);


void namespace_OpaqueIterable_destroy(OpaqueIterable* self);





#endif // OpaqueIterable_H
