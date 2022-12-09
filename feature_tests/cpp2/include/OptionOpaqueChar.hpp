#ifndef OptionOpaqueChar_HPP
#define OptionOpaqueChar_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "OptionOpaqueChar.d.hpp"
#include "OptionOpaqueChar.h"




void inline OptionOpaqueChar::assert_char(char32_t ch) const {
	// TODO
}
inline capi::OptionOpaqueChar* OptionOpaqueChar::AsFFI() {
	return reinterpret_cast<capi::OptionOpaqueChar*>(this);
}
inline OptionOpaqueChar::~OptionOpaqueChar() {
	capi::OptionOpaqueChar_destroy(AsFFI());
}


#endif // OptionOpaqueChar_HPP
