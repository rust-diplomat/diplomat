#ifndef StructArithmetic_D_HPP
#define StructArithmetic_D_HPP

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
    struct StructArithmetic {
      int32_t x;
      int32_t y;
    };
    
    typedef struct StructArithmetic_option {union { StructArithmetic ok; }; bool is_ok; } StructArithmetic_option;
} // namespace capi
} // namespace


struct StructArithmetic {
  int32_t x;
  int32_t y;

  inline static StructArithmetic ORIGIN();

  inline static void set_origin(StructArithmetic _new_origin);

  inline static StructArithmetic new_(int32_t x, int32_t y);

  inline StructArithmetic operator+(StructArithmetic o) const;
  inline StructArithmetic& operator+=(StructArithmetic o);

  inline StructArithmetic operator-(StructArithmetic o) const;
  inline StructArithmetic& operator-=(StructArithmetic o);

  inline StructArithmetic operator*(StructArithmetic o) const;
  inline StructArithmetic& operator*=(StructArithmetic o);

  inline StructArithmetic operator/(StructArithmetic o) const;
  inline StructArithmetic& operator/=(StructArithmetic o);

  inline diplomat::capi::StructArithmetic AsFFI() const;
  inline static StructArithmetic FromFFI(diplomat::capi::StructArithmetic c_struct);
};


#endif // StructArithmetic_D_HPP
