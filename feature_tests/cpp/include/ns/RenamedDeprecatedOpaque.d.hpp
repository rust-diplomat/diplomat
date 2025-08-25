#ifndef ns_RenamedDeprecatedOpaque_D_HPP
#define ns_RenamedDeprecatedOpaque_D_HPP

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
    struct RenamedDeprecatedOpaque;
} // namespace capi
} // namespace

namespace ns {
/**
 * \deprecated use Foo
 */
class [[deprecated("use Foo")]] RenamedDeprecatedOpaque {
public:

  inline const ns::capi::RenamedDeprecatedOpaque* AsFFI() const;
  inline ns::capi::RenamedDeprecatedOpaque* AsFFI();
  inline static const ns::RenamedDeprecatedOpaque* FromFFI(const ns::capi::RenamedDeprecatedOpaque* ptr);
  inline static ns::RenamedDeprecatedOpaque* FromFFI(ns::capi::RenamedDeprecatedOpaque* ptr);
  inline static void operator delete(void* ptr);
private:
  RenamedDeprecatedOpaque() = delete;
  RenamedDeprecatedOpaque(const ns::RenamedDeprecatedOpaque&) = delete;
  RenamedDeprecatedOpaque(ns::RenamedDeprecatedOpaque&&) noexcept = delete;
  RenamedDeprecatedOpaque operator=(const ns::RenamedDeprecatedOpaque&) = delete;
  RenamedDeprecatedOpaque operator=(ns::RenamedDeprecatedOpaque&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ns_RenamedDeprecatedOpaque_D_HPP
