#ifndef StructWithSlices_D_HPP
#define StructWithSlices_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    struct StructWithSlices {
      diplomat::capi::DiplomatStringView first;
      diplomat::capi::DiplomatU16View second;
    };

    typedef struct StructWithSlices_option {union { StructWithSlices ok; }; bool is_ok; } StructWithSlices_option;

} // namespace capi
} // namespace


struct StructWithSlices {
  std::string_view first;
  diplomat::span<const uint16_t> second;

  inline std::string return_last() const;

  inline diplomat::capi::StructWithSlices AsFFI() const;
  inline static StructWithSlices FromFFI(diplomat::capi::StructWithSlices c_struct);
};


#endif // StructWithSlices_D_HPP
