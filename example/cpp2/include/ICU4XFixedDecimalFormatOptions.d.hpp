#ifndef ICU4XFixedDecimalFormatOptions_D_HPP
#define ICU4XFixedDecimalFormatOptions_D_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XFixedDecimalGroupingStrategy.d.hpp"
#include "ICU4XFixedDecimalSignDisplay.d.hpp"




struct ICU4XFixedDecimalFormatOptions {
	ICU4XFixedDecimalGroupingStrategy grouping_strategy;
	ICU4XFixedDecimalSignDisplay sign_display;
};





#endif // ICU4XFixedDecimalFormatOptions_D_HPP
