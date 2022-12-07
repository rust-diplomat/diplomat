#ifndef ImportedStruct_H
#define ImportedStruct_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "UnimportedEnum.h"




typedef struct ImportedStruct {
	UnimportedEnum foo;
	uint8_t count;
} ImportedStruct;





#endif // ImportedStruct_HPP
