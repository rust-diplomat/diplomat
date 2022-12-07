#ifndef ICU4XFixedDecimalFormatResult_HPP
#define ICU4XFixedDecimalFormatResult_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


class ICU4XFixedDecimalFormat;


struct ICU4XFixedDecimalFormatResult {
	std::unique_ptr<ICU4XFixedDecimalFormat> fdf;
	bool success;
};





#endif // ICU4XFixedDecimalFormatResult_HPP
