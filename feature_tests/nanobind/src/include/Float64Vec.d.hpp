#ifndef SOMELIB_Float64Vec_D_HPP
#define SOMELIB_Float64Vec_D_HPP

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
namespace capi { struct Float64Vec; }
class Float64Vec;
} // namespace somelib



namespace somelib {
namespace capi {
    struct Float64Vec;
} // namespace capi
} // namespace

namespace somelib {
class Float64Vec {
public:

  inline static std::unique_ptr<somelib::Float64Vec> new_(somelib::diplomat::span<const double> v);

  inline static std::unique_ptr<somelib::Float64Vec> new_bool(somelib::diplomat::span<const bool> v);

  inline static std::unique_ptr<somelib::Float64Vec> new_i16(somelib::diplomat::span<const int16_t> v);

  inline static std::unique_ptr<somelib::Float64Vec> new_u16(somelib::diplomat::span<const uint16_t> v);

  inline static std::unique_ptr<somelib::Float64Vec> new_isize(somelib::diplomat::span<const intptr_t> v);

  inline static std::unique_ptr<somelib::Float64Vec> new_usize(somelib::diplomat::span<const size_t> v);

  inline static std::unique_ptr<somelib::Float64Vec> new_f64_be_bytes(somelib::diplomat::span<const uint8_t> v);

  inline somelib::diplomat::span<const double> as_slice() const;

  inline void fill_slice(somelib::diplomat::span<double> v) const;

  inline void set_value(somelib::diplomat::span<const double> new_slice);

  inline std::string to_string() const;
  template<typename W>
  inline void to_string_write(W& writeable_output) const;

  inline somelib::diplomat::span<const double> borrow() const;

  inline std::optional<double> operator[](size_t i) const;

    inline const somelib::capi::Float64Vec* AsFFI() const;
    inline somelib::capi::Float64Vec* AsFFI();
    inline static const somelib::Float64Vec* FromFFI(const somelib::capi::Float64Vec* ptr);
    inline static somelib::Float64Vec* FromFFI(somelib::capi::Float64Vec* ptr);
    inline static void operator delete(void* ptr);
private:
    Float64Vec() = delete;
    Float64Vec(const somelib::Float64Vec&) = delete;
    Float64Vec(somelib::Float64Vec&&) noexcept = delete;
    Float64Vec operator=(const somelib::Float64Vec&) = delete;
    Float64Vec operator=(somelib::Float64Vec&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_Float64Vec_D_HPP
