#ifndef OptionOpaque_HPP
#define OptionOpaque_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "OptionOpaque.d.hpp"
#include "OptionOpaque.h"




static std::unique_ptr<OptionOpaque> inline OptionOpaque::new_(int32_t i) {
	// TODO
}
static std::unique_ptr<OptionOpaque> inline OptionOpaque::new_none() {
	// TODO
}
static OptionStruct inline OptionOpaque::new_struct() {
	// TODO
}
static OptionStruct inline OptionOpaque::new_struct_nones() {
	// TODO
}
void inline OptionOpaque::assert_integer(int32_t i) const {
	// TODO
}
static bool inline OptionOpaque::option_opaque_argument(const std::optional<OptionOpaque&> arg) {
	// TODO
}
inline capi::OptionOpaque* OptionOpaque::AsFFI() {
	return reinterpret_cast<capi::OptionOpaque*>(this);
}
inline OptionOpaque::~OptionOpaque() {
	capi::OptionOpaque_destroy(AsFFI());
}


#endif // OptionOpaque_HPP
