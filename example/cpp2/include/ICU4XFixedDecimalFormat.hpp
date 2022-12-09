#ifndef ICU4XFixedDecimalFormat_HPP
#define ICU4XFixedDecimalFormat_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




	static ICU4XFixedDecimalFormatResult inline try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XFixedDecimalFormatOptions options) {
	// TODO
}
	std::string inline format_write(const ICU4XFixedDecimal& value) const {
	// TODO
}
inline capi::ICU4XFixedDecimalFormat* AsFFI() {
	return reinterpret_cast::<capi::ICU4XFixedDecimalFormat>(this);
}
inline ~ICU4XFixedDecimalFormat() {
	capi::ICU4XFixedDecimalFormat_destroy(AsFFI());
}


#endif // ICU4XFixedDecimalFormat_HPP
