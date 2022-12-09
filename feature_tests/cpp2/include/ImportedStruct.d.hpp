#ifndef ImportedStruct_D_HPP
#define ImportedStruct_D_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "UnimportedEnum.d.hpp"




struct ImportedStruct {
	UnimportedEnum foo;
	uint8_t count;
};





#endif // ImportedStruct_D_HPP
