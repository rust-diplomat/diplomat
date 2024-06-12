#ifndef CPPRenamedAttrOpaque2_D_HPP
#define CPPRenamedAttrOpaque2_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef struct AttrOpaque2 AttrOpaque2;
}

namespace ns {
class CPPRenamedAttrOpaque2 {
public:

  inline const capi::AttrOpaque2* AsFFI() const;
  inline capi::AttrOpaque2* AsFFI();
  inline static const ns::CPPRenamedAttrOpaque2* FromFFI(const capi::AttrOpaque2* ptr);
  inline static ns::CPPRenamedAttrOpaque2* FromFFI(capi::AttrOpaque2* ptr);
  inline static void operator delete(void* ptr);
private:
  CPPRenamedAttrOpaque2() = delete;
  CPPRenamedAttrOpaque2(const ns::CPPRenamedAttrOpaque2&) = delete;
  CPPRenamedAttrOpaque2(ns::CPPRenamedAttrOpaque2&&) noexcept = delete;
  CPPRenamedAttrOpaque2 operator=(const ns::CPPRenamedAttrOpaque2&) = delete;
  CPPRenamedAttrOpaque2 operator=(ns::CPPRenamedAttrOpaque2&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

}
#endif // CPPRenamedAttrOpaque2_D_HPP
