#ifndef Float64VecError_D_HPP
#define Float64VecError_D_HPP

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
    struct Float64VecError;
} // namespace capi
} // namespace

class Float64VecError {
public:

  inline static std::unique_ptr<Float64VecError> new_(diplomat::span<const double> v);

  inline diplomat::result<double, std::monostate> operator[](size_t i) const;

  inline const diplomat::capi::Float64VecError* AsFFI() const;
  inline diplomat::capi::Float64VecError* AsFFI();
  inline static const Float64VecError* FromFFI(const diplomat::capi::Float64VecError* ptr);
  inline static Float64VecError* FromFFI(diplomat::capi::Float64VecError* ptr);
  inline static void operator delete(void* ptr);
private:
  Float64VecError() = delete;
  Float64VecError(const Float64VecError&) = delete;
  Float64VecError(Float64VecError&&) noexcept = delete;
  Float64VecError operator=(const Float64VecError&) = delete;
  Float64VecError operator=(Float64VecError&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Float64VecError_D_HPP
