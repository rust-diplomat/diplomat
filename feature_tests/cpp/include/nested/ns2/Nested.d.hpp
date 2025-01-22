#ifndef nested_ns2_Nested_D_HPP
#define nested_ns2_Nested_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../../diplomat_runtime.hpp"


namespace nested::ns2 {
namespace capi {
    struct Nested;
} // namespace capi
} // namespace

namespace nested::ns2 {
class Nested {
public:

  inline const nested::ns2::capi::Nested* AsFFI() const;
  inline nested::ns2::capi::Nested* AsFFI();
  inline static const nested::ns2::Nested* FromFFI(const nested::ns2::capi::Nested* ptr);
  inline static nested::ns2::Nested* FromFFI(nested::ns2::capi::Nested* ptr);
  inline static void operator delete(void* ptr);
private:
  Nested() = delete;
  Nested(const nested::ns2::Nested&) = delete;
  Nested(nested::ns2::Nested&&) noexcept = delete;
  Nested operator=(const nested::ns2::Nested&) = delete;
  Nested operator=(nested::ns2::Nested&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // nested_ns2_Nested_D_HPP
