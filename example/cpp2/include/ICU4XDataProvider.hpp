#ifndef ICU4XDataProvider_HPP
#define ICU4XDataProvider_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




	static std::unique_ptr<ICU4XDataProvider> inline new_static() {
	// TODO
}
	static DiplomatResult<std::monostate, std::monostate> inline returns_result() {
	// TODO
}
inline capi::ICU4XDataProvider* AsFFI() {
	return reinterpret_cast::<capi::ICU4XDataProvider>(this);
}
inline ~ICU4XDataProvider() {
	capi::ICU4XDataProvider_destroy(AsFFI());
}


#endif // ICU4XDataProvider_HPP
