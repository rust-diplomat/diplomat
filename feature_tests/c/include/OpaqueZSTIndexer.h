#ifndef OpaqueZSTIndexer_H
#define OpaqueZSTIndexer_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "OpaqueZSTIndexer.d.h"






OpaqueZSTIndexer* namespace_OpaqueZSTIndexer_new(void);

OpaqueZSTIndexer* namespace_OpaqueZSTIndexer_index(const OpaqueZSTIndexer* self, size_t idx);

void namespace_OpaqueZSTIndexer_destroy(OpaqueZSTIndexer* self);





#endif // OpaqueZSTIndexer_H
