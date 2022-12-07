#ifndef ICU4XFixedDecimalFormatOptions_H
#define ICU4XFixedDecimalFormatOptions_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XFixedDecimalGroupingStrategy.hpp"
#include "ICU4XFixedDecimalSignDisplay.hpp"




struct ICU4XFixedDecimalFormatOptions {
	ICU4XFixedDecimalGroupingStrategy grouping_strategy;
	ICU4XFixedDecimalSignDisplay sign_display;
};



ICU4XFixedDecimalFormatOptions ICU4XFixedDecimalFormatOptions_default();


#endif // ICU4XFixedDecimalFormatOptions_HPP
