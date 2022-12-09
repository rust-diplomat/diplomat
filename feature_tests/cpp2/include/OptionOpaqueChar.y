#ifndef OptionOpaqueChar_HPP
#define OptionOpaqueChar_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




class OptionOpaqueChar {
public:
	void assert_char(char32_t ch) const;

	inline capi::OptionOpaqueChar AsFFI() {
		return reinterpret_cast::<capi::OptionOpaqueChar>(this);
	}

	~OptionOpaqueChar() {
		capi::OptionOpaqueChar_destroy(AsFFI());
	}

private:
	OptionOpaqueChar() = delete;
}





#endif // OptionOpaqueChar_HPP
