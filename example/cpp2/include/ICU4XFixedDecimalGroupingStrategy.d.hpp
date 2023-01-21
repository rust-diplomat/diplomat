#ifndef ICU4XFixedDecimalGroupingStrategy_D_HPP
#define ICU4XFixedDecimalGroupingStrategy_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XFixedDecimalGroupingStrategy.d.h"


class ICU4XFixedDecimalGroupingStrategy {
  capi::ICU4XFixedDecimalGroupingStrategy value;

public:
  enum Value {
    Auto,
    Never,
    Always,
    Min2,
  };

  inline ICU4XFixedDecimalGroupingStrategy(ICU4XFixedDecimalGroupingStrategy::Value cpp_value);
  inline ICU4XFixedDecimalGroupingStrategy(capi::ICU4XFixedDecimalGroupingStrategy c_enum) : value(c_enum) {};

  inline capi::ICU4XFixedDecimalGroupingStrategy AsFFI() const;
  inline static ICU4XFixedDecimalGroupingStrategy FromFFI(capi::ICU4XFixedDecimalGroupingStrategy c_enum);
};


#endif // ICU4XFixedDecimalGroupingStrategy_D_HPP
