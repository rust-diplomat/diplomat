#ifndef Float64Vec_D_HPP
#define Float64Vec_D_HPP

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
    struct Float64Vec;
} // namespace capi
} // namespace

class Float64Vec {
public:

  inline static std::unique_ptr<Float64Vec> new_(diplomat::span<const double> v);

  inline static std::unique_ptr<Float64Vec> new_bool(diplomat::span<const bool> v);

  inline static std::unique_ptr<Float64Vec> new_i16(diplomat::span<const int16_t> v);

  inline static std::unique_ptr<Float64Vec> new_u16(diplomat::span<const uint16_t> v);

  inline static std::unique_ptr<Float64Vec> new_isize(diplomat::span<const intptr_t> v);

  inline static std::unique_ptr<Float64Vec> new_usize(diplomat::span<const size_t> v);

  inline static std::unique_ptr<Float64Vec> new_f64_be_bytes(diplomat::span<const uint8_t> v);

  inline diplomat::span<const double> as_slice() const;

  inline void fill_slice(diplomat::span<double> v) const;

  inline void set_value(diplomat::span<const double> new_slice);

  inline std::string to_string() const;

  inline diplomat::span<const double> borrow() const;

  inline std::optional<double> operator[](size_t i) const;

  inline const diplomat::capi::Float64Vec* AsFFI() const;
  inline diplomat::capi::Float64Vec* AsFFI();
  inline static const Float64Vec* FromFFI(const diplomat::capi::Float64Vec* ptr);
  inline static Float64Vec* FromFFI(diplomat::capi::Float64Vec* ptr);
  inline static void operator delete(void* ptr);
private:
  Float64Vec() = delete;
  Float64Vec(const Float64Vec&) = delete;
  Float64Vec(Float64Vec&&) noexcept = delete;
  Float64Vec operator=(const Float64Vec&) = delete;
  Float64Vec operator=(Float64Vec&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Float64Vec_D_HPP
