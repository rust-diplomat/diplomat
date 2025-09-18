#ifndef SOMELIB_ns_RenamedComparable_D_HPP
#define SOMELIB_ns_RenamedComparable_D_HPP

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
namespace capi { struct RenamedComparable; }
class RenamedComparable;
} // namespace ns
} // namespace somelib



namespace somelib::ns {
namespace capi {
    struct RenamedComparable;
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedComparable {
public:

  inline static std::unique_ptr<somelib::ns::RenamedComparable> new_(uint8_t int_);

  inline int8_t cmp(const somelib::ns::RenamedComparable& other) const;
  inline bool operator==(const somelib::ns::RenamedComparable& other) const;
  inline bool operator!=(const somelib::ns::RenamedComparable& other) const;
  inline bool operator<=(const somelib::ns::RenamedComparable& other) const;
  inline bool operator>=(const somelib::ns::RenamedComparable& other) const;
  inline bool operator<(const somelib::ns::RenamedComparable& other) const;
  inline bool operator>(const somelib::ns::RenamedComparable& other) const;

    inline const somelib::ns::capi::RenamedComparable* AsFFI() const;
    inline somelib::ns::capi::RenamedComparable* AsFFI();
    inline static const somelib::ns::RenamedComparable* FromFFI(const somelib::ns::capi::RenamedComparable* ptr);
    inline static somelib::ns::RenamedComparable* FromFFI(somelib::ns::capi::RenamedComparable* ptr);
    inline static void operator delete(void* ptr);
private:
    RenamedComparable() = delete;
    RenamedComparable(const somelib::ns::RenamedComparable&) = delete;
    RenamedComparable(somelib::ns::RenamedComparable&&) noexcept = delete;
    RenamedComparable operator=(const somelib::ns::RenamedComparable&) = delete;
    RenamedComparable operator=(somelib::ns::RenamedComparable&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_ns_RenamedComparable_D_HPP
