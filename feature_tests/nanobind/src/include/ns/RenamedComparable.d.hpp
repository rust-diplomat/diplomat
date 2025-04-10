#ifndef ns_RenamedComparable_D_HPP
#define ns_RenamedComparable_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"

namespace ns {
namespace capi { struct RenamedComparable; }
class RenamedComparable;
}


namespace ns {
namespace capi {
    struct RenamedComparable;
} // namespace capi
} // namespace

namespace ns {
class RenamedComparable {
public:

  inline static std::unique_ptr<ns::RenamedComparable> new_(uint8_t int_);

  inline int8_t cmp(const ns::RenamedComparable& other) const;
  inline bool operator==(const ns::RenamedComparable& other) const;
  inline bool operator!=(const ns::RenamedComparable& other) const;
  inline bool operator<=(const ns::RenamedComparable& other) const;
  inline bool operator>=(const ns::RenamedComparable& other) const;
  inline bool operator<(const ns::RenamedComparable& other) const;
  inline bool operator>(const ns::RenamedComparable& other) const;

  inline const ns::capi::RenamedComparable* AsFFI() const;
  inline ns::capi::RenamedComparable* AsFFI();
  inline static const ns::RenamedComparable* FromFFI(const ns::capi::RenamedComparable* ptr);
  inline static ns::RenamedComparable* FromFFI(ns::capi::RenamedComparable* ptr);
  inline static void operator delete(void* ptr);
private:
  RenamedComparable() = delete;
  RenamedComparable(const ns::RenamedComparable&) = delete;
  RenamedComparable(ns::RenamedComparable&&) noexcept = delete;
  RenamedComparable operator=(const ns::RenamedComparable&) = delete;
  RenamedComparable operator=(ns::RenamedComparable&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ns_RenamedComparable_D_HPP
