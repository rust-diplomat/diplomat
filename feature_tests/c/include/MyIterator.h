#ifndef MyIterator_H
#define MyIterator_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "MyIterator.d.h"






typedef struct namespace_MyIterator_next_result {union {uint8_t ok; }; bool is_ok;} namespace_MyIterator_next_result;
namespace_MyIterator_next_result namespace_MyIterator_next(MyIterator* self);


void namespace_MyIterator_destroy(MyIterator* self);





#endif // MyIterator_H
