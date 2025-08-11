#ifndef ns_RenamedTestOpaque_D_HPP
#define ns_RenamedTestOpaque_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"


namespace ns {
namespace capi {
    struct RenamedTestOpaque;
} // namespace capi
} // namespace

namespace ns {
class RenamedTestOpaque {
public:

  inline const ns::capi::RenamedTestOpaque* AsFFI() const;
  inline ns::capi::RenamedTestOpaque* AsFFI();
  inline static const ns::RenamedTestOpaque* FromFFI(const ns::capi::RenamedTestOpaque* ptr);
  inline static ns::RenamedTestOpaque* FromFFI(ns::capi::RenamedTestOpaque* ptr);
  inline static void operator delete(void* ptr);
private:
  RenamedTestOpaque() = delete;
  RenamedTestOpaque(const ns::RenamedTestOpaque&) = delete;
  RenamedTestOpaque(ns::RenamedTestOpaque&&) noexcept = delete;
  RenamedTestOpaque operator=(const ns::RenamedTestOpaque&) = delete;
  RenamedTestOpaque operator=(ns::RenamedTestOpaque&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ns_RenamedTestOpaque_D_HPP
