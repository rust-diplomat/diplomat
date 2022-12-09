#ifndef ImportedStruct_HPP
#define ImportedStruct_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "UnimportedEnum.y"




struct ImportedStruct {
	UnimportedEnum foo;
	uint8_t count;
};





#endif // ImportedStruct_HPP
