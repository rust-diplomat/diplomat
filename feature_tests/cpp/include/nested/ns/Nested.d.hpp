#ifndef nested_ns_Nested_D_HPP
#define nested_ns_Nested_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../../diplomat_runtime.hpp"


namespace nested::ns {
namespace capi {
    struct Nested;
} // namespace capi
} // namespace

namespace nested::ns {
class Nested {
public:

  inline const nested::ns::capi::Nested* AsFFI() const;
  inline nested::ns::capi::Nested* AsFFI();
  inline static const nested::ns::Nested* FromFFI(const nested::ns::capi::Nested* ptr);
  inline static nested::ns::Nested* FromFFI(nested::ns::capi::Nested* ptr);
  inline static void operator delete(void* ptr);
private:
  Nested() = delete;
  Nested(const nested::ns::Nested&) = delete;
  Nested(nested::ns::Nested&&) noexcept = delete;
  Nested operator=(const nested::ns::Nested&) = delete;
  Nested operator=(nested::ns::Nested&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // nested_ns_Nested_D_HPP
