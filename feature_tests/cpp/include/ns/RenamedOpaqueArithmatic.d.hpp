#ifndef ns_RenamedOpaqueArithmatic_D_HPP
#define ns_RenamedOpaqueArithmatic_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"

namespace ns {
namespace capi { struct RenamedOpaqueArithmatic; }
class RenamedOpaqueArithmatic;
}


namespace ns {
namespace capi {
    struct RenamedOpaqueArithmatic;
} // namespace capi
} // namespace

namespace ns {
class RenamedOpaqueArithmatic {
public:

  inline static std::unique_ptr<ns::RenamedOpaqueArithmatic> make(int32_t x, int32_t y);

  inline int32_t x() const;

  inline int32_t y() const;

  inline std::unique_ptr<ns::RenamedOpaqueArithmatic> operator+(const ns::RenamedOpaqueArithmatic& o) const;

  inline std::unique_ptr<ns::RenamedOpaqueArithmatic> operator-(const ns::RenamedOpaqueArithmatic& o) const;

  inline std::unique_ptr<ns::RenamedOpaqueArithmatic> operator*(const ns::RenamedOpaqueArithmatic& o) const;

  inline std::unique_ptr<ns::RenamedOpaqueArithmatic> operator/(const ns::RenamedOpaqueArithmatic& o) const;

  inline void operator+=(const ns::RenamedOpaqueArithmatic& o);

  inline void operator-=(const ns::RenamedOpaqueArithmatic& o);

  inline void operator*=(const ns::RenamedOpaqueArithmatic& o);

  inline void operator/=(const ns::RenamedOpaqueArithmatic& o);

  inline const ns::capi::RenamedOpaqueArithmatic* AsFFI() const;
  inline ns::capi::RenamedOpaqueArithmatic* AsFFI();
  inline static const ns::RenamedOpaqueArithmatic* FromFFI(const ns::capi::RenamedOpaqueArithmatic* ptr);
  inline static ns::RenamedOpaqueArithmatic* FromFFI(ns::capi::RenamedOpaqueArithmatic* ptr);
  inline static void operator delete(void* ptr);
private:
  RenamedOpaqueArithmatic() = delete;
  RenamedOpaqueArithmatic(const ns::RenamedOpaqueArithmatic&) = delete;
  RenamedOpaqueArithmatic(ns::RenamedOpaqueArithmatic&&) noexcept = delete;
  RenamedOpaqueArithmatic operator=(const ns::RenamedOpaqueArithmatic&) = delete;
  RenamedOpaqueArithmatic operator=(ns::RenamedOpaqueArithmatic&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ns_RenamedOpaqueArithmatic_D_HPP
