#ifndef OpaqueIterator_H
#define OpaqueIterator_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "AttrOpaque1.d.h"

#include "OpaqueIterator.d.h"






AttrOpaque1* namespace_OpaqueIterator_next(OpaqueIterator* self);


void namespace_OpaqueIterator_destroy(OpaqueIterator* self);





#endif // OpaqueIterator_H
