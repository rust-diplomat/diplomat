#ifndef CPPRenamedAttrOpaque2_D_HPP
#define CPPRenamedAttrOpaque2_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace ns {
namespace capi {
    struct CPPRenamedAttrOpaque2;
} // namespace capi
} // namespace

namespace ns {
class CPPRenamedAttrOpaque2 {
public:

  inline const ns::capi::CPPRenamedAttrOpaque2* AsFFI() const;
  inline ns::capi::CPPRenamedAttrOpaque2* AsFFI();
  inline static const ns::CPPRenamedAttrOpaque2* FromFFI(const ns::capi::CPPRenamedAttrOpaque2* ptr);
  inline static ns::CPPRenamedAttrOpaque2* FromFFI(ns::capi::CPPRenamedAttrOpaque2* ptr);
  inline static void operator delete(void* ptr);
private:
  CPPRenamedAttrOpaque2() = delete;
  CPPRenamedAttrOpaque2(const ns::CPPRenamedAttrOpaque2&) = delete;
  CPPRenamedAttrOpaque2(ns::CPPRenamedAttrOpaque2&&) noexcept = delete;
  CPPRenamedAttrOpaque2 operator=(const ns::CPPRenamedAttrOpaque2&) = delete;
  CPPRenamedAttrOpaque2 operator=(ns::CPPRenamedAttrOpaque2&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // CPPRenamedAttrOpaque2_D_HPP
