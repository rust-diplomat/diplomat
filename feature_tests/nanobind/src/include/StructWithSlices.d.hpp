#ifndef SOMELIB_StructWithSlices_D_HPP
#define SOMELIB_StructWithSlices_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    struct StructWithSlices {
      somelib::diplomat::capi::DiplomatStringView first;
      somelib::diplomat::capi::DiplomatU16View second;
    };

    typedef struct StructWithSlices_option {union { StructWithSlices ok; }; bool is_ok; } StructWithSlices_option;
} // namespace capi
} // namespace


namespace somelib {
struct StructWithSlices {
    std::string_view first;
    somelib::diplomat::span<const uint16_t> second;

  inline std::string return_last() const;
  template<typename W>
  inline void return_last_write(W& writeable_output) const;

    inline somelib::capi::StructWithSlices AsFFI() const;
    inline static somelib::StructWithSlices FromFFI(somelib::capi::StructWithSlices c_struct);
};

} // namespace
#endif // SOMELIB_StructWithSlices_D_HPP
