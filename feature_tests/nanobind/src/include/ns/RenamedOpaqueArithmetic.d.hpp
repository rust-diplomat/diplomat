#ifndef SOMELIB_ns_RenamedOpaqueArithmetic_D_HPP
#define SOMELIB_ns_RenamedOpaqueArithmetic_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
namespace somelib {
namespace ns {
namespace capi { struct RenamedOpaqueArithmetic; }
class RenamedOpaqueArithmetic;
} // namespace ns
} // namespace somelib



namespace somelib::ns {
namespace capi {
    struct RenamedOpaqueArithmetic;
    extern "C" {
    void namespace_OpaqueArithmetic_destroy(RenamedOpaqueArithmetic* self);
    }
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedOpaqueArithmetic;
using RenamedOpaqueArithmeticRef = somelib::diplomat::Ref<RenamedOpaqueArithmetic, const somelib::ns::capi::RenamedOpaqueArithmetic>;
using RenamedOpaqueArithmeticRefMut = somelib::diplomat::Ref<RenamedOpaqueArithmetic, somelib::ns::capi::RenamedOpaqueArithmetic>;

class RenamedOpaqueArithmetic : public somelib::diplomat::OpaquePointer<RenamedOpaqueArithmetic, somelib::ns::capi::RenamedOpaqueArithmetic, somelib::ns::capi::namespace_OpaqueArithmetic_destroy> {
public:

  inline static somelib::ns::RenamedOpaqueArithmetic make(int32_t x, int32_t y = 12);

  inline static somelib::ns::RenamedOpaqueArithmetic make(float x, float y = 14.48, somelib::diplomat::Optional<float> z = somelib::diplomat::Optional<float>(0));

  inline static somelib::ns::RenamedOpaqueArithmetic make(float x, bool z);

  inline int32_t x() const;

  inline int32_t x(int32_t add) const;

  inline int32_t y() const;

  inline somelib::ns::RenamedOpaqueArithmetic operator+(const somelib::ns::RenamedOpaqueArithmetic& o) const;

  inline somelib::ns::RenamedOpaqueArithmetic operator-(const somelib::ns::RenamedOpaqueArithmetic& o) const;

  inline somelib::ns::RenamedOpaqueArithmetic operator*(const somelib::ns::RenamedOpaqueArithmetic& o) const;

  inline somelib::ns::RenamedOpaqueArithmetic operator/(const somelib::ns::RenamedOpaqueArithmetic& o) const;

  inline void operator+=(const somelib::ns::RenamedOpaqueArithmetic& o);

  inline void operator-=(const somelib::ns::RenamedOpaqueArithmetic& o);

  inline void operator*=(const somelib::ns::RenamedOpaqueArithmetic& o);

  inline void operator/=(const somelib::ns::RenamedOpaqueArithmetic& o);

};

} // namespace
#endif // SOMELIB_ns_RenamedOpaqueArithmetic_D_HPP
