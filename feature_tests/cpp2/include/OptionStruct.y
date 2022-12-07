#ifndef OptionStruct_H
#define OptionStruct_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "OptionOpaque.h"
#include "OptionOpaqueChar.h"


class OptionOpaque;
class OptionOpaqueChar;


typedef struct OptionStruct {
	std::unique_ptr<OptionOpaque> a;
	std::unique_ptr<OptionOpaqueChar> b;
	uint32_t c;
	std::unique_ptr<OptionOpaque> d;
} OptionStruct;





#endif // OptionStruct_HPP
