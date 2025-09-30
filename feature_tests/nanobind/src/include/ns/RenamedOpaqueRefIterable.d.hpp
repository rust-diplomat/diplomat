#ifndef SOMELIB_ns_RenamedOpaqueRefIterable_D_HPP
#define SOMELIB_ns_RenamedOpaqueRefIterable_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
namespace somelib {
namespace ns {
namespace capi { struct RenamedOpaqueRefIterable; }
class RenamedOpaqueRefIterable;
namespace capi { struct RenamedOpaqueRefIterator; }
class RenamedOpaqueRefIterator;
} // namespace ns
} // namespace somelib



namespace somelib::ns {
namespace capi {
    struct RenamedOpaqueRefIterable;
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedOpaqueRefIterable {
public:

  inline static std::unique_ptr<somelib::ns::RenamedOpaqueRefIterable> new_(size_t size);

  inline std::unique_ptr<somelib::ns::RenamedOpaqueRefIterator> iter() const;
  inline somelib::diplomat::next_to_iter_helper<somelib::ns::RenamedOpaqueRefIterator> begin() const;
  inline std::nullopt_t end() const { return std::nullopt; }

    inline const somelib::ns::capi::RenamedOpaqueRefIterable* AsFFI() const;
    inline somelib::ns::capi::RenamedOpaqueRefIterable* AsFFI();
    inline static const somelib::ns::RenamedOpaqueRefIterable* FromFFI(const somelib::ns::capi::RenamedOpaqueRefIterable* ptr);
    inline static somelib::ns::RenamedOpaqueRefIterable* FromFFI(somelib::ns::capi::RenamedOpaqueRefIterable* ptr);
    inline static void operator delete(void* ptr);
private:
    RenamedOpaqueRefIterable() = delete;
    RenamedOpaqueRefIterable(const somelib::ns::RenamedOpaqueRefIterable&) = delete;
    RenamedOpaqueRefIterable(somelib::ns::RenamedOpaqueRefIterable&&) noexcept = delete;
    RenamedOpaqueRefIterable operator=(const somelib::ns::RenamedOpaqueRefIterable&) = delete;
    RenamedOpaqueRefIterable operator=(somelib::ns::RenamedOpaqueRefIterable&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_ns_RenamedOpaqueRefIterable_D_HPP
