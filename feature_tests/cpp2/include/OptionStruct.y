#ifndef OptionStruct_HPP
#define OptionStruct_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


class OptionOpaque;
class OptionOpaqueChar;


struct OptionStruct {
	std::unique_ptr<OptionOpaque> a;
	std::unique_ptr<OptionOpaqueChar> b;
	uint32_t c;
	std::unique_ptr<OptionOpaque> d;
};





#endif // OptionStruct_HPP
