#ifndef StructWithSlices_HPP
#define StructWithSlices_HPP

#include "StructWithSlices.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    void StructWithSlices_return_last(diplomat::capi::StructWithSlices self, diplomat::capi::DiplomatWrite* write);
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::string StructWithSlices::return_last() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::StructWithSlices_return_last(this->AsFFI(),
    &write);
  return output;
}


inline diplomat::capi::StructWithSlices StructWithSlices::AsFFI() const {
  return diplomat::capi::StructWithSlices {
    /* .first = */ {first.data(), first.size()},
    /* .second = */ {second.data(), second.size()},
  };
}

inline StructWithSlices StructWithSlices::FromFFI(diplomat::capi::StructWithSlices c_struct) {
  return StructWithSlices {
    /* .first = */ std::string_view(c_struct.first.data, c_struct.first.len),
    /* .second = */ diplomat::span<const uint16_t>(c_struct.second.data, c_struct.second.len),
  };
}


#endif // StructWithSlices_HPP
