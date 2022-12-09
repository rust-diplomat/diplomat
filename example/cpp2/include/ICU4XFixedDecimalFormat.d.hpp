#ifndef ICU4XFixedDecimalFormat_D_HPP
#define ICU4XFixedDecimalFormat_D_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XFixedDecimalFormat.d.h"
#include "ICU4XFixedDecimalFormatOptions.d.hpp"
#include "ICU4XFixedDecimalFormatResult.d.hpp"


class ICU4XDataProvider;
class ICU4XFixedDecimal;
class ICU4XLocale;


class ICU4XFixedDecimalFormat {
public:
	static ICU4XFixedDecimalFormatResult inline try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XFixedDecimalFormatOptions options);

	std::string inline format_write(const ICU4XFixedDecimal& value) const;

	inline capi::ICU4XFixedDecimalFormat* AsFFI();

	inline ~ICU4XFixedDecimalFormat();

private:
	ICU4XFixedDecimalFormat() = delete;
};





#endif // ICU4XFixedDecimalFormat_D_HPP
