#ifndef ICU4XFixedDecimalFormatOptions_D_HPP
#define ICU4XFixedDecimalFormatOptions_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XFixedDecimalGroupingStrategy.d.hpp"
#include "ICU4XFixedDecimalSignDisplay.d.hpp"

enum struct ICU4XFixedDecimalGroupingStrategy;
enum struct ICU4XFixedDecimalSignDisplay;


struct ICU4XFixedDecimalFormatOptions {
  ICU4XFixedDecimalGroupingStrategy grouping_strategy;
  ICU4XFixedDecimalSignDisplay sign_display;
};


#endif // ICU4XFixedDecimalFormatOptions_D_HPP
