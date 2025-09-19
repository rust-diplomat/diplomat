#ifndef SOMELIB_StructArithmetic_D_HPP
#define SOMELIB_StructArithmetic_D_HPP

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
struct StructArithmetic;
} // namespace somelib



namespace somelib {
namespace capi {
    struct StructArithmetic {
      int32_t x;
      int32_t y;
    };

    typedef struct StructArithmetic_option {union { StructArithmetic ok; }; bool is_ok; } StructArithmetic_option;
} // namespace capi
} // namespace


namespace somelib {
struct StructArithmetic {
    int32_t x;
    int32_t y;

  inline static somelib::StructArithmetic ORIGIN();

  inline static void set_origin(somelib::StructArithmetic _new_origin);

  inline static somelib::StructArithmetic new_(int32_t x, int32_t y);

  inline somelib::StructArithmetic operator+(somelib::StructArithmetic o) const;
  inline somelib::StructArithmetic& operator+=(somelib::StructArithmetic o);

  inline somelib::StructArithmetic operator-(somelib::StructArithmetic o) const;
  inline somelib::StructArithmetic& operator-=(somelib::StructArithmetic o);

  inline somelib::StructArithmetic operator*(somelib::StructArithmetic o) const;
  inline somelib::StructArithmetic& operator*=(somelib::StructArithmetic o);

  inline somelib::StructArithmetic operator/(somelib::StructArithmetic o) const;
  inline somelib::StructArithmetic& operator/=(somelib::StructArithmetic o);

    inline somelib::capi::StructArithmetic AsFFI() const;
    inline static somelib::StructArithmetic FromFFI(somelib::capi::StructArithmetic c_struct);
};

} // namespace
#endif // SOMELIB_StructArithmetic_D_HPP
