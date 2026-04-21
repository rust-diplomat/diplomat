#ifndef SOMELIB_StructOfOpaque_D_HPP
#define SOMELIB_StructOfOpaque_D_HPP

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
namespace capi { struct OpaqueMut; }
class OpaqueMut;
} // namespace somelib



namespace somelib {
namespace capi {
    struct StructOfOpaque {
      const somelib::capi::Opaque* i;
      somelib::capi::OpaqueMut* j;
    };

    typedef struct StructOfOpaque_option {union { StructOfOpaque ok; }; bool is_ok; } StructOfOpaque_option;
} // namespace capi
} // namespace


namespace somelib {
struct StructOfOpaque {
    somelib::Opaque* i;
    somelib::OpaqueMut* j;

  inline void take_in(const somelib::Opaque& other);

    inline somelib::capi::StructOfOpaque AsFFI() const;
    inline static somelib::StructOfOpaque FromFFI(somelib::capi::StructOfOpaque c_struct);
};

} // namespace
#endif // SOMELIB_StructOfOpaque_D_HPP
