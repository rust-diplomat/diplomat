#ifndef MyStruct_H
#define MyStruct_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "MyEnum.hpp"




struct MyStruct {
	uint8_t a;
	bool b;
	uint8_t c;
	uint64_t d;
	int32_t e;
	char32_t f;
	MyEnum g;
};



MyStruct MyStruct_new();


#endif // MyStruct_HPP
