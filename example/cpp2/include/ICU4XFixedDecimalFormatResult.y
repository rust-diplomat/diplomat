#ifndef ICU4XFixedDecimalFormatResult_H
#define ICU4XFixedDecimalFormatResult_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XFixedDecimalFormat.h"


class ICU4XFixedDecimalFormat;


typedef struct ICU4XFixedDecimalFormatResult {
	std::unique_ptr<ICU4XFixedDecimalFormat> fdf;
	bool success;
} ICU4XFixedDecimalFormatResult;





#endif // ICU4XFixedDecimalFormatResult_HPP
