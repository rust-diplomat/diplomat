#ifndef ImportedStruct_HPP
#define ImportedStruct_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ImportedStruct.h"

#include "UnimportedEnum.hpp"

/**
 * A destruction policy for using ImportedStruct with std::unique_ptr.
 */
struct ImportedStructDeleter {
  void operator()(capi::ImportedStruct* l) const noexcept {
    capi::ImportedStruct_destroy(l);
  }
};
struct ImportedStruct {
 public:
  UnimportedEnum foo;
  uint8_t int;
};


#endif
