#ifndef ns_RenamedAttrOpaque2_D_HPP
#define ns_RenamedAttrOpaque2_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace ns {
namespace capi {
    struct RenamedAttrOpaque2;
} // namespace capi
} // namespace

namespace ns {
class RenamedAttrOpaque2 {
public:

  inline const ns::capi::RenamedAttrOpaque2* AsFFI() const;
  inline ns::capi::RenamedAttrOpaque2* AsFFI();
  inline static const ns::RenamedAttrOpaque2* FromFFI(const ns::capi::RenamedAttrOpaque2* ptr);
  inline static ns::RenamedAttrOpaque2* FromFFI(ns::capi::RenamedAttrOpaque2* ptr);
  inline static void operator delete(void* ptr);
private:
  RenamedAttrOpaque2() = delete;
  RenamedAttrOpaque2(const ns::RenamedAttrOpaque2&) = delete;
  RenamedAttrOpaque2(ns::RenamedAttrOpaque2&&) noexcept = delete;
  RenamedAttrOpaque2 operator=(const ns::RenamedAttrOpaque2&) = delete;
  RenamedAttrOpaque2 operator=(ns::RenamedAttrOpaque2&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ns_RenamedAttrOpaque2_D_HPP
