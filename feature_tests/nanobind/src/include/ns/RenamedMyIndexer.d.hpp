#ifndef ns_RenamedMyIndexer_D_HPP
#define ns_RenamedMyIndexer_D_HPP

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
    struct RenamedMyIndexer;
} // namespace capi
} // namespace

namespace ns {
class RenamedMyIndexer {
public:

  inline std::optional<std::string_view> operator[](size_t i) const;

  inline const ns::capi::RenamedMyIndexer* AsFFI() const;
  inline ns::capi::RenamedMyIndexer* AsFFI();
  inline static const ns::RenamedMyIndexer* FromFFI(const ns::capi::RenamedMyIndexer* ptr);
  inline static ns::RenamedMyIndexer* FromFFI(ns::capi::RenamedMyIndexer* ptr);
  inline static void operator delete(void* ptr);
private:
  RenamedMyIndexer() = delete;
  RenamedMyIndexer(const ns::RenamedMyIndexer&) = delete;
  RenamedMyIndexer(ns::RenamedMyIndexer&&) noexcept = delete;
  RenamedMyIndexer operator=(const ns::RenamedMyIndexer&) = delete;
  RenamedMyIndexer operator=(ns::RenamedMyIndexer&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ns_RenamedMyIndexer_D_HPP
