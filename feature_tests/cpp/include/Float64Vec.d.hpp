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

    typedef struct DiplomatFloat64VecView {
      const Float64Vec** data;
      size_t len;
    } DiplomatFloat64VecView;
    extern "C" {
    void Float64Vec_destroy(Float64Vec* self);
    }
} // namespace capi
} // namespace

namespace somelib {
class Float64Vec;
using Float64VecRef = somelib::diplomat::Ref<Float64Vec, const somelib::capi::Float64Vec>;
using Float64VecRefMut = somelib::diplomat::Ref<Float64Vec, somelib::capi::Float64Vec>;

class Float64Vec : public somelib::diplomat::OpaquePointer<Float64Vec, somelib::capi::Float64Vec, somelib::capi::Float64Vec_destroy> {
public:

  inline static somelib::Float64Vec new_(somelib::diplomat::span<const double> v);

  inline static somelib::Float64Vec new_bool(somelib::diplomat::span<const bool> v);

  inline static somelib::Float64Vec new_i16(somelib::diplomat::span<const int16_t> v);

  inline static somelib::Float64Vec new_u16(somelib::diplomat::span<const uint16_t> v);

  inline static somelib::Float64Vec new_isize(somelib::diplomat::span<const intptr_t> v);

  inline static somelib::Float64Vec new_usize(somelib::diplomat::span<const size_t> v);

  inline static somelib::Float64Vec new_f64_be_bytes(somelib::diplomat::span<const uint8_t> v);

  inline somelib::diplomat::span<const double> as_slice() const DIPLOMAT_LIFETIME_BOUND;

  inline void fill_slice(somelib::diplomat::span<double> v) const;

  inline void set_value(somelib::diplomat::span<const double> new_slice);

  inline std::string to_string() const;
  template<typename W>
  inline void to_string_write(W& writeable_output) const;

  inline somelib::diplomat::span<const double> borrow() const DIPLOMAT_LIFETIME_BOUND;

  inline somelib::diplomat::Optional<double> operator[](size_t i) const;

};

} // namespace
#endif // SOMELIB_Float64Vec_D_HPP
