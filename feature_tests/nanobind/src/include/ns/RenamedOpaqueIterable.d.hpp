#ifndef ns_RenamedOpaqueIterable_D_HPP
#define ns_RenamedOpaqueIterable_D_HPP

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
namespace capi { struct RenamedOpaqueIterable; }
class RenamedOpaqueIterable;
namespace capi { struct RenamedOpaqueIterator; }
class RenamedOpaqueIterator;
} // namespace ns




namespace ns {
namespace capi {
    struct RenamedOpaqueIterable;
} // namespace capi
} // namespace

namespace ns {
class RenamedOpaqueIterable {
public:

  inline static std::unique_ptr<ns::RenamedOpaqueIterable> new_(size_t size);

  inline std::unique_ptr<ns::RenamedOpaqueIterator> iter() const;
  inline diplomat::next_to_iter_helper<ns::RenamedOpaqueIterator> begin() const;
  inline std::nullopt_t end() const { return std::nullopt; }

    inline const ns::capi::RenamedOpaqueIterable* AsFFI() const;
    inline ns::capi::RenamedOpaqueIterable* AsFFI();
    inline static const ns::RenamedOpaqueIterable* FromFFI(const ns::capi::RenamedOpaqueIterable* ptr);
    inline static ns::RenamedOpaqueIterable* FromFFI(ns::capi::RenamedOpaqueIterable* ptr);
    inline static void operator delete(void* ptr);
private:
    RenamedOpaqueIterable() = delete;
    RenamedOpaqueIterable(const ns::RenamedOpaqueIterable&) = delete;
    RenamedOpaqueIterable(ns::RenamedOpaqueIterable&&) noexcept = delete;
    RenamedOpaqueIterable operator=(const ns::RenamedOpaqueIterable&) = delete;
    RenamedOpaqueIterable operator=(ns::RenamedOpaqueIterable&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ns_RenamedOpaqueIterable_D_HPP
