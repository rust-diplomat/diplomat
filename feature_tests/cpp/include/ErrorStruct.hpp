#ifndef ErrorStruct_HPP
#define ErrorStruct_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ErrorStruct.h"
}


/**
 * A destruction policy for using ErrorStruct with std::unique_ptr.
 */
struct ErrorStructDeleter {
  void operator()(capi::ErrorStruct* l) const noexcept {
    capi::ErrorStruct_destroy(l);
  }
};
struct ErrorStruct {
 public:
  int32_t i;
  int32_t j;
};


#endif
