#ifndef ns_RenamedOpaqueRefIterable_D_HPP
#define ns_RenamedOpaqueRefIterable_D_HPP

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
namespace capi { struct RenamedOpaqueRefIterable; }
class RenamedOpaqueRefIterable;
namespace capi { struct RenamedOpaqueRefIterator; }
class RenamedOpaqueRefIterator;
} // namespace ns




namespace ns {
namespace capi {
    struct RenamedOpaqueRefIterable;
} // namespace capi
} // namespace

namespace ns {
class RenamedOpaqueRefIterable {
public:

  inline static std::unique_ptr<ns::RenamedOpaqueRefIterable> new_(size_t size);

  inline std::unique_ptr<ns::RenamedOpaqueRefIterator> iter() const;
  inline diplomat::next_to_iter_helper<ns::RenamedOpaqueRefIterator> begin() const;
  inline std::nullopt_t end() const { return std::nullopt; }

    inline const ns::capi::RenamedOpaqueRefIterable* AsFFI() const;
    inline ns::capi::RenamedOpaqueRefIterable* AsFFI();
    inline static const ns::RenamedOpaqueRefIterable* FromFFI(const ns::capi::RenamedOpaqueRefIterable* ptr);
    inline static ns::RenamedOpaqueRefIterable* FromFFI(ns::capi::RenamedOpaqueRefIterable* ptr);
    inline static void operator delete(void* ptr);
private:
    RenamedOpaqueRefIterable() = delete;
    RenamedOpaqueRefIterable(const ns::RenamedOpaqueRefIterable&) = delete;
    RenamedOpaqueRefIterable(ns::RenamedOpaqueRefIterable&&) noexcept = delete;
    RenamedOpaqueRefIterable operator=(const ns::RenamedOpaqueRefIterable&) = delete;
    RenamedOpaqueRefIterable operator=(ns::RenamedOpaqueRefIterable&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ns_RenamedOpaqueRefIterable_D_HPP
