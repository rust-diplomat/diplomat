#ifndef SOMELIB_ns_RenamedMyIterator_D_HPP
#define SOMELIB_ns_RenamedMyIterator_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"


namespace somelib::ns {
namespace capi {
    struct RenamedMyIterator;
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedMyIterator {
public:

  inline std::optional<uint8_t> next();

    inline const somelib::ns::capi::RenamedMyIterator* AsFFI() const;
    inline somelib::ns::capi::RenamedMyIterator* AsFFI();
    inline static const somelib::ns::RenamedMyIterator* FromFFI(const somelib::ns::capi::RenamedMyIterator* ptr);
    inline static somelib::ns::RenamedMyIterator* FromFFI(somelib::ns::capi::RenamedMyIterator* ptr);
    inline static void operator delete(void* ptr);
private:
    RenamedMyIterator() = delete;
    RenamedMyIterator(const somelib::ns::RenamedMyIterator&) = delete;
    RenamedMyIterator(somelib::ns::RenamedMyIterator&&) noexcept = delete;
    RenamedMyIterator operator=(const somelib::ns::RenamedMyIterator&) = delete;
    RenamedMyIterator operator=(somelib::ns::RenamedMyIterator&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_ns_RenamedMyIterator_D_HPP
