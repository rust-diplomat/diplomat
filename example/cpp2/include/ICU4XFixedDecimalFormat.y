#ifndef ICU4XFixedDecimalFormat_H
#define ICU4XFixedDecimalFormat_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XFixedDecimalFormatOptions.hpp"
#include "ICU4XFixedDecimalFormatResult.hpp"


class ICU4XDataProvider;
class ICU4XFixedDecimal;
class ICU4XLocale;


class ICU4XFixedDecimalFormat {
public:
	static ICU4XFixedDecimalFormatResult try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XFixedDecimalFormatOptions options);

	std::string format_write(const ICU4XFixedDecimal& value);

	inline capi::ICU4XFixedDecimalFormat AsFFI() {
		return reinterpret_cast::<capi::ICU4XFixedDecimalFormat>(this);
	}

	~ICU4XFixedDecimalFormat() {
		ICU4XFixedDecimalFormat_destroy(AsFFI());
	}

private:
	ICU4XFixedDecimalFormat() = delete;
}





#endif // ICU4XFixedDecimalFormat_HPP
