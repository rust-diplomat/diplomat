#ifndef OptionOpaque_HPP
#define OptionOpaque_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




	static std::unique_ptr<OptionOpaque> inline new_(int32_t i) {
	// TODO
}
	static std::unique_ptr<OptionOpaque> inline new_none() {
	// TODO
}
	static OptionStruct inline new_struct() {
	// TODO
}
	static OptionStruct inline new_struct_nones() {
	// TODO
}
	void inline assert_integer(int32_t i) const {
	// TODO
}
	static bool inline option_opaque_argument(const std::optional<OptionOpaque&> arg) {
	// TODO
}
inline capi::OptionOpaque* AsFFI() {
	return reinterpret_cast::<capi::OptionOpaque>(this);
}
inline ~OptionOpaque() {
	capi::OptionOpaque_destroy(AsFFI());
}


#endif // OptionOpaque_HPP
