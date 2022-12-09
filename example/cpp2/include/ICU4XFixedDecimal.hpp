#ifndef ICU4XFixedDecimal_HPP
#define ICU4XFixedDecimal_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




	static std::unique_ptr<ICU4XFixedDecimal> inline new_(int32_t v) {
	// TODO
}
	void inline multiply_pow10(int16_t power) {
	// TODO
}
	void inline negate() {
	// TODO
}
	DiplomatResult<std::string, std::monostate> inline to_string() const {
	// TODO
}
inline capi::ICU4XFixedDecimal* AsFFI() {
	return reinterpret_cast::<capi::ICU4XFixedDecimal>(this);
}
inline ~ICU4XFixedDecimal() {
	capi::ICU4XFixedDecimal_destroy(AsFFI());
}


#endif // ICU4XFixedDecimal_HPP
