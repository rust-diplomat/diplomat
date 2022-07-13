#ifndef ICU4XFixedDecimalGroupingStrategy_HPP
#define ICU4XFixedDecimalGroupingStrategy_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XFixedDecimalGroupingStrategy.h"
}


enum struct ICU4XFixedDecimalGroupingStrategy {

  /**
   * Auto grouping
   */
  Auto = 0,

  /**
   * No grouping
   */
  Never = 1,

  /**
   * Always group
   */
  Always = 2,

  /**
   * At least 2 groups
   */
  Min2 = 3,
};

#endif
