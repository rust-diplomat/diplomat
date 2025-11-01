#ifndef SOMELIB_ns_RenamedMyIndexer_D_HPP
#define SOMELIB_ns_RenamedMyIndexer_D_HPP

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
    struct RenamedMyIndexer;
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedMyIndexer {
public:

  inline std::optional<std::string_view> operator[](size_t i) const;

    inline const somelib::ns::capi::RenamedMyIndexer* AsFFI() const;
    inline somelib::ns::capi::RenamedMyIndexer* AsFFI();
    inline static const somelib::ns::RenamedMyIndexer* FromFFI(const somelib::ns::capi::RenamedMyIndexer* ptr);
    inline static somelib::ns::RenamedMyIndexer* FromFFI(somelib::ns::capi::RenamedMyIndexer* ptr);
    inline static void operator delete(void* ptr);
private:
    RenamedMyIndexer() = delete;
    RenamedMyIndexer(const somelib::ns::RenamedMyIndexer&) = delete;
    RenamedMyIndexer(somelib::ns::RenamedMyIndexer&&) noexcept = delete;
    RenamedMyIndexer operator=(const somelib::ns::RenamedMyIndexer&) = delete;
    RenamedMyIndexer operator=(somelib::ns::RenamedMyIndexer&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_ns_RenamedMyIndexer_D_HPP
