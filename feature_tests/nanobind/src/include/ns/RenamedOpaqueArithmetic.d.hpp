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
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedOpaqueArithmetic {
public:

  inline static std::unique_ptr<somelib::ns::RenamedOpaqueArithmetic> make(int32_t x, int32_t y);

  inline static std::unique_ptr<somelib::ns::RenamedOpaqueArithmetic> make(float x, float y);

  inline int32_t x() const;

  inline int32_t y() const;

  inline std::unique_ptr<somelib::ns::RenamedOpaqueArithmetic> operator+(const somelib::ns::RenamedOpaqueArithmetic& o) const;

  inline std::unique_ptr<somelib::ns::RenamedOpaqueArithmetic> operator-(const somelib::ns::RenamedOpaqueArithmetic& o) const;

  inline std::unique_ptr<somelib::ns::RenamedOpaqueArithmetic> operator*(const somelib::ns::RenamedOpaqueArithmetic& o) const;

  inline std::unique_ptr<somelib::ns::RenamedOpaqueArithmetic> operator/(const somelib::ns::RenamedOpaqueArithmetic& o) const;

  inline void operator+=(const somelib::ns::RenamedOpaqueArithmetic& o);

  inline void operator-=(const somelib::ns::RenamedOpaqueArithmetic& o);

  inline void operator*=(const somelib::ns::RenamedOpaqueArithmetic& o);

  inline void operator/=(const somelib::ns::RenamedOpaqueArithmetic& o);

    inline const somelib::ns::capi::RenamedOpaqueArithmetic* AsFFI() const;
    inline somelib::ns::capi::RenamedOpaqueArithmetic* AsFFI();
    inline static const somelib::ns::RenamedOpaqueArithmetic* FromFFI(const somelib::ns::capi::RenamedOpaqueArithmetic* ptr);
    inline static somelib::ns::RenamedOpaqueArithmetic* FromFFI(somelib::ns::capi::RenamedOpaqueArithmetic* ptr);
    inline static void operator delete(void* ptr);
private:
    RenamedOpaqueArithmetic() = delete;
    RenamedOpaqueArithmetic(const somelib::ns::RenamedOpaqueArithmetic&) = delete;
    RenamedOpaqueArithmetic(somelib::ns::RenamedOpaqueArithmetic&&) noexcept = delete;
    RenamedOpaqueArithmetic operator=(const somelib::ns::RenamedOpaqueArithmetic&) = delete;
    RenamedOpaqueArithmetic operator=(somelib::ns::RenamedOpaqueArithmetic&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_ns_RenamedOpaqueArithmetic_D_HPP
