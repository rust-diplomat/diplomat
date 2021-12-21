#ifndef ErrorEnum_HPP
#define ErrorEnum_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ErrorEnum.h"
}


enum struct ErrorEnum {
  Foo = 0,
  Bar = 1,
};

#endif
