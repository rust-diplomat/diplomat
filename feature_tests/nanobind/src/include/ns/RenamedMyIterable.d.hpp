#ifndef SOMELIB_ns_RenamedMyIterable_D_HPP
#define SOMELIB_ns_RenamedMyIterable_D_HPP

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
namespace capi { struct RenamedMyIterable; }
class RenamedMyIterable;
namespace capi { struct RenamedMyIterator; }
class RenamedMyIterator;
} // namespace ns
} // namespace somelib



namespace somelib::ns {
namespace capi {
    struct RenamedMyIterable;
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedMyIterable {
public:

  inline static std::unique_ptr<somelib::ns::RenamedMyIterable> new_(somelib::diplomat::span<const uint8_t> x);

  inline std::unique_ptr<somelib::ns::RenamedMyIterator> iter() const;
  inline somelib::diplomat::next_to_iter_helper<somelib::ns::RenamedMyIterator> begin() const;
  inline std::nullopt_t end() const { return std::nullopt; }

  inline size_t __len__() const;

    inline const somelib::ns::capi::RenamedMyIterable* AsFFI() const;
    inline somelib::ns::capi::RenamedMyIterable* AsFFI();
    inline static const somelib::ns::RenamedMyIterable* FromFFI(const somelib::ns::capi::RenamedMyIterable* ptr);
    inline static somelib::ns::RenamedMyIterable* FromFFI(somelib::ns::capi::RenamedMyIterable* ptr);
    inline static void operator delete(void* ptr);
private:
    RenamedMyIterable() = delete;
    RenamedMyIterable(const somelib::ns::RenamedMyIterable&) = delete;
    RenamedMyIterable(somelib::ns::RenamedMyIterable&&) noexcept = delete;
    RenamedMyIterable operator=(const somelib::ns::RenamedMyIterable&) = delete;
    RenamedMyIterable operator=(somelib::ns::RenamedMyIterable&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_ns_RenamedMyIterable_D_HPP
