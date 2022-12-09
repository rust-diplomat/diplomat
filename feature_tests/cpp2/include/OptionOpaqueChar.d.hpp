#ifndef OptionOpaqueChar_D_HPP
#define OptionOpaqueChar_D_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "OptionOpaqueChar.d.h"




class OptionOpaqueChar {
public:
	void inline assert_char(char32_t ch) const;

	inline capi::OptionOpaqueChar* AsFFI();

	inline ~OptionOpaqueChar();

private:
	OptionOpaqueChar() = delete;
};





#endif // OptionOpaqueChar_D_HPP
