#ifndef SOMELIB_ImmutableStructOfOpaque_D_HPP
#define SOMELIB_ImmutableStructOfOpaque_D_HPP

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
namespace capi { struct Opaque; }
class Opaque;
} // namespace somelib



namespace somelib {
namespace capi {
    struct ImmutableStructOfOpaque {
      const somelib::capi::Opaque* i;
    };

    typedef struct ImmutableStructOfOpaque_option {union { ImmutableStructOfOpaque ok; }; bool is_ok; } ImmutableStructOfOpaque_option;
} // namespace capi
} // namespace


namespace somelib {
struct ImmutableStructOfOpaque {
    const somelib::Opaque& i;

  inline std::string take_in() const;
  template<typename W>
  inline void take_in_write(W& writeable_output) const;

    inline somelib::capi::ImmutableStructOfOpaque AsFFI() const;
    inline static somelib::ImmutableStructOfOpaque FromFFI(somelib::capi::ImmutableStructOfOpaque c_struct);
};

} // namespace
#endif // SOMELIB_ImmutableStructOfOpaque_D_HPP
