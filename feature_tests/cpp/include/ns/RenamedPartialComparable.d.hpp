#ifndef SOMELIB_ns_RenamedPartialComparable_D_HPP
#define SOMELIB_ns_RenamedPartialComparable_D_HPP

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
namespace capi { struct RenamedPartialComparable; }
class RenamedPartialComparable;
} // namespace ns
} // namespace somelib



namespace somelib::ns {
namespace capi {
    struct RenamedPartialComparable;
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedPartialComparable {
public:

  inline static std::unique_ptr<somelib::ns::RenamedPartialComparable> new_(float float_);

  inline std::optional<int8_t> partial_cmp(const somelib::ns::RenamedPartialComparable& other) const;

  inline std::optional<bool> operator==(const somelib::ns::RenamedPartialComparable& other) const;
  inline std::optional<bool> operator!=(const somelib::ns::RenamedPartialComparable& other) const;
  inline std::optional<bool> operator<=(const somelib::ns::RenamedPartialComparable& other) const;
  inline std::optional<bool> operator>=(const somelib::ns::RenamedPartialComparable& other) const;
  inline std::optional<bool> operator<(const somelib::ns::RenamedPartialComparable& other) const;
  inline std::optional<bool> operator>(const somelib::ns::RenamedPartialComparable& other) const;

  inline std::optional<int8_t> test_nonstd(const somelib::ns::RenamedPartialComparable& other) const;

    inline const somelib::ns::capi::RenamedPartialComparable* AsFFI() const;
    inline somelib::ns::capi::RenamedPartialComparable* AsFFI();
    inline static const somelib::ns::RenamedPartialComparable* FromFFI(const somelib::ns::capi::RenamedPartialComparable* ptr);
    inline static somelib::ns::RenamedPartialComparable* FromFFI(somelib::ns::capi::RenamedPartialComparable* ptr);
    inline static void operator delete(void* ptr);
private:
    RenamedPartialComparable() = delete;
    RenamedPartialComparable(const somelib::ns::RenamedPartialComparable&) = delete;
    RenamedPartialComparable(somelib::ns::RenamedPartialComparable&&) noexcept = delete;
    RenamedPartialComparable operator=(const somelib::ns::RenamedPartialComparable&) = delete;
    RenamedPartialComparable operator=(somelib::ns::RenamedPartialComparable&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_ns_RenamedPartialComparable_D_HPP
