#ifndef AttrEnum_HPP
#define AttrEnum_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "AttrEnum.h"


enum struct AttrEnum {
  A = 0,
  B = 1,
  C = 2,
};

#endif
