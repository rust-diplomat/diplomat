#ifndef OptionOpaque_D_HPP
#define OptionOpaque_D_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "OptionOpaque.d.h"
#include "OptionStruct.d.hpp"




class OptionOpaque {
public:
	static std::unique_ptr<OptionOpaque> inline new_(int32_t i);

	static std::unique_ptr<OptionOpaque> inline new_none();

	static OptionStruct inline new_struct();

	static OptionStruct inline new_struct_nones();

	void inline assert_integer(int32_t i) const;

	static bool inline option_opaque_argument(const std::optional<OptionOpaque&> arg);

	inline capi::OptionOpaque* AsFFI();

	inline ~OptionOpaque();

private:
	OptionOpaque() = delete;
};





#endif // OptionOpaque_D_HPP
