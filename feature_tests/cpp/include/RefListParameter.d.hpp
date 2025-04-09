#ifndef RefListParameter_D_HPP
#define RefListParameter_D_HPP

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
    struct RefListParameter;
} // namespace capi
} // namespace

class RefListParameter {
public:

  inline const diplomat::capi::RefListParameter* AsFFI() const;
  inline diplomat::capi::RefListParameter* AsFFI();
  inline static const RefListParameter* FromFFI(const diplomat::capi::RefListParameter* ptr);
  inline static RefListParameter* FromFFI(diplomat::capi::RefListParameter* ptr);
  inline static void operator delete(void* ptr);
private:
  RefListParameter() = delete;
  RefListParameter(const RefListParameter&) = delete;
  RefListParameter(RefListParameter&&) noexcept = delete;
  RefListParameter operator=(const RefListParameter&) = delete;
  RefListParameter operator=(RefListParameter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // RefListParameter_D_HPP
