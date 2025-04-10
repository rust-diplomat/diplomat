#ifndef ns_RenamedMyIterable_D_HPP
#define ns_RenamedMyIterable_D_HPP

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
namespace capi { struct RenamedMyIterable; }
class RenamedMyIterable;
namespace capi { struct RenamedMyIterator; }
class RenamedMyIterator;
}


namespace ns {
namespace capi {
    struct RenamedMyIterable;

} // namespace capi
} // namespace

namespace ns {
class RenamedMyIterable {
public:

  inline static std::unique_ptr<ns::RenamedMyIterable> new_(diplomat::span<const uint8_t> x);

  inline std::unique_ptr<ns::RenamedMyIterator> iter() const;
  inline diplomat::next_to_iter_helper<ns::RenamedMyIterator> begin() const;
  inline std::nullopt_t end() const { return std::nullopt; }

  inline const ns::capi::RenamedMyIterable* AsFFI() const;
  inline ns::capi::RenamedMyIterable* AsFFI();
  inline static const ns::RenamedMyIterable* FromFFI(const ns::capi::RenamedMyIterable* ptr);
  inline static ns::RenamedMyIterable* FromFFI(ns::capi::RenamedMyIterable* ptr);
  inline static void operator delete(void* ptr);
private:
  RenamedMyIterable() = delete;
  RenamedMyIterable(const ns::RenamedMyIterable&) = delete;
  RenamedMyIterable(ns::RenamedMyIterable&&) noexcept = delete;
  RenamedMyIterable operator=(const ns::RenamedMyIterable&) = delete;
  RenamedMyIterable operator=(ns::RenamedMyIterable&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ns_RenamedMyIterable_D_HPP
