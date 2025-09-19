#ifndef SOMELIB_ns_RenamedDeprecatedOpaque_D_HPP
#define SOMELIB_ns_RenamedDeprecatedOpaque_D_HPP

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
    struct RenamedDeprecatedOpaque;
} // namespace capi
} // namespace

namespace somelib::ns {
/**
 * \deprecated use Foo
 */
class [[deprecated("use Foo")]] RenamedDeprecatedOpaque {
public:

    inline const somelib::ns::capi::RenamedDeprecatedOpaque* AsFFI() const;
    inline somelib::ns::capi::RenamedDeprecatedOpaque* AsFFI();
    inline static const somelib::ns::RenamedDeprecatedOpaque* FromFFI(const somelib::ns::capi::RenamedDeprecatedOpaque* ptr);
    inline static somelib::ns::RenamedDeprecatedOpaque* FromFFI(somelib::ns::capi::RenamedDeprecatedOpaque* ptr);
    inline static void operator delete(void* ptr);
private:
    RenamedDeprecatedOpaque() = delete;
    RenamedDeprecatedOpaque(const somelib::ns::RenamedDeprecatedOpaque&) = delete;
    RenamedDeprecatedOpaque(somelib::ns::RenamedDeprecatedOpaque&&) noexcept = delete;
    RenamedDeprecatedOpaque operator=(const somelib::ns::RenamedDeprecatedOpaque&) = delete;
    RenamedDeprecatedOpaque operator=(somelib::ns::RenamedDeprecatedOpaque&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_ns_RenamedDeprecatedOpaque_D_HPP
