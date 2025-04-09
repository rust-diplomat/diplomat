#ifndef Two_D_HPP
#define Two_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    struct Two;
} // namespace capi
} // namespace

class Two {
public:

  inline const diplomat::capi::Two* AsFFI() const;
  inline diplomat::capi::Two* AsFFI();
  inline static const Two* FromFFI(const diplomat::capi::Two* ptr);
  inline static Two* FromFFI(diplomat::capi::Two* ptr);
  inline static void operator delete(void* ptr);
private:
  Two() = delete;
  Two(const Two&) = delete;
  Two(Two&&) noexcept = delete;
  Two operator=(const Two&) = delete;
  Two operator=(Two&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Two_D_HPP
