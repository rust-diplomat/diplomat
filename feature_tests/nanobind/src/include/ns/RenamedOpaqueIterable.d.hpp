#ifndef SOMELIB_ns_RenamedOpaqueIterable_D_HPP
#define SOMELIB_ns_RenamedOpaqueIterable_D_HPP

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
namespace capi { struct RenamedOpaqueIterable; }
class RenamedOpaqueIterable;
namespace capi { struct RenamedOpaqueIterator; }
class RenamedOpaqueIterator;
} // namespace ns
} // namespace somelib



namespace somelib::ns {
namespace capi {
    struct RenamedOpaqueIterable;
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedOpaqueIterable {
public:

  inline static std::unique_ptr<somelib::ns::RenamedOpaqueIterable> new_(size_t size);

  inline std::unique_ptr<somelib::ns::RenamedOpaqueIterator> iter() const;
  inline somelib::diplomat::next_to_iter_helper<somelib::ns::RenamedOpaqueIterator> begin() const;
  inline std::nullopt_t end() const { return std::nullopt; }

    inline const somelib::ns::capi::RenamedOpaqueIterable* AsFFI() const;
    inline somelib::ns::capi::RenamedOpaqueIterable* AsFFI();
    inline static const somelib::ns::RenamedOpaqueIterable* FromFFI(const somelib::ns::capi::RenamedOpaqueIterable* ptr);
    inline static somelib::ns::RenamedOpaqueIterable* FromFFI(somelib::ns::capi::RenamedOpaqueIterable* ptr);
    inline static void operator delete(void* ptr);
private:
    RenamedOpaqueIterable() = delete;
    RenamedOpaqueIterable(const somelib::ns::RenamedOpaqueIterable&) = delete;
    RenamedOpaqueIterable(somelib::ns::RenamedOpaqueIterable&&) noexcept = delete;
    RenamedOpaqueIterable operator=(const somelib::ns::RenamedOpaqueIterable&) = delete;
    RenamedOpaqueIterable operator=(somelib::ns::RenamedOpaqueIterable&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_ns_RenamedOpaqueIterable_D_HPP
