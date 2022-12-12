#ifndef ICU4XFixedDecimalFormatResult_D_HPP
#define ICU4XFixedDecimalFormatResult_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

class ICU4XFixedDecimalFormat;


struct ICU4XFixedDecimalFormatResult {
  std::unique_ptr<ICU4XFixedDecimalFormat> fdf;
  bool success;
};


#endif // ICU4XFixedDecimalFormatResult_D_HPP
