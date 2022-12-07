#ifndef OptionOpaque_H
#define OptionOpaque_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "OptionStruct.hpp"




class OptionOpaque {
public:
	static std::unique_ptr<OptionOpaque> new_(int32_t i);

	static std::unique_ptr<OptionOpaque> new_none();

	static OptionStruct new_struct();

	static OptionStruct new_struct_nones();

	void assert_integer(int32_t i);

	static bool option_opaque_argument(const std::optional<OptionOpaque&> arg);

	inline capi::OptionOpaque AsFFI() {
		return reinterpret_cast::<capi::OptionOpaque>(this);
	}

	~OptionOpaque() {
		OptionOpaque_destroy(AsFFI());
	}

private:
	OptionOpaque() = delete;
}





#endif // OptionOpaque_HPP
