#ifndef ns_RenamedOpaqueArithmetic_D_HPP
#define ns_RenamedOpaqueArithmetic_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"

namespace ns {
namespace capi { struct RenamedOpaqueArithmetic; }
class RenamedOpaqueArithmetic;
}


namespace ns {
namespace capi {
    struct RenamedOpaqueArithmetic;
} // namespace capi
} // namespace

namespace ns {
class RenamedOpaqueArithmetic {
public:

  inline static std::unique_ptr<ns::RenamedOpaqueArithmetic> make(int32_t x, int32_t y);

  inline int32_t x() const;

  inline int32_t y() const;

  inline std::unique_ptr<ns::RenamedOpaqueArithmetic> operator+(const ns::RenamedOpaqueArithmetic& o) const;

  inline std::unique_ptr<ns::RenamedOpaqueArithmetic> operator-(const ns::RenamedOpaqueArithmetic& o) const;

  inline std::unique_ptr<ns::RenamedOpaqueArithmetic> operator*(const ns::RenamedOpaqueArithmetic& o) const;

  inline std::unique_ptr<ns::RenamedOpaqueArithmetic> operator/(const ns::RenamedOpaqueArithmetic& o) const;

  inline void operator+=(const ns::RenamedOpaqueArithmetic& o);

  inline void operator-=(const ns::RenamedOpaqueArithmetic& o);

  inline void operator*=(const ns::RenamedOpaqueArithmetic& o);

  inline void operator/=(const ns::RenamedOpaqueArithmetic& o);

  inline const ns::capi::RenamedOpaqueArithmetic* AsFFI() const;
  inline ns::capi::RenamedOpaqueArithmetic* AsFFI();
  inline static const ns::RenamedOpaqueArithmetic* FromFFI(const ns::capi::RenamedOpaqueArithmetic* ptr);
  inline static ns::RenamedOpaqueArithmetic* FromFFI(ns::capi::RenamedOpaqueArithmetic* ptr);
  inline static void operator delete(void* ptr);
private:
  RenamedOpaqueArithmetic() = delete;
  RenamedOpaqueArithmetic(const ns::RenamedOpaqueArithmetic&) = delete;
  RenamedOpaqueArithmetic(ns::RenamedOpaqueArithmetic&&) noexcept = delete;
  RenamedOpaqueArithmetic operator=(const ns::RenamedOpaqueArithmetic&) = delete;
  RenamedOpaqueArithmetic operator=(ns::RenamedOpaqueArithmetic&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ns_RenamedOpaqueArithmetic_D_HPP
