#ifndef VectorTest_H
#define VectorTest_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "VectorTest.d.h"






VectorTest* namespace_VectorTest_new(void);

size_t namespace_VectorTest_len(const VectorTest* self);

typedef struct namespace_VectorTest_get_result {union {double ok; }; bool is_ok;} namespace_VectorTest_get_result;
namespace_VectorTest_get_result namespace_VectorTest_get(const VectorTest* self, size_t idx);

void namespace_VectorTest_push(VectorTest* self, double value);

void namespace_VectorTest_destroy(VectorTest* self);





#endif // VectorTest_H
