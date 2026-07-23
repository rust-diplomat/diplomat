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
    extern "C" {
    void namespace_PartialComparable_destroy(RenamedPartialComparable* self);
    }
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedPartialComparable;
using RenamedPartialComparableRef = somelib::diplomat::Ref<RenamedPartialComparable, const somelib::ns::capi::RenamedPartialComparable>;
using RenamedPartialComparableRefMut = somelib::diplomat::Ref<RenamedPartialComparable, somelib::ns::capi::RenamedPartialComparable>;

class RenamedPartialComparable : public somelib::diplomat::OpaquePointer<RenamedPartialComparable, somelib::ns::capi::RenamedPartialComparable, somelib::ns::capi::namespace_PartialComparable_destroy> {
public:

  inline static somelib::ns::RenamedPartialComparable new_(float float_);

  inline somelib::diplomat::Optional<int8_t> partial_cmp(const somelib::ns::RenamedPartialComparable& other) const;

  inline std::optional<bool> operator==(const somelib::ns::RenamedPartialComparable& other) const;
  inline std::optional<bool> operator!=(const somelib::ns::RenamedPartialComparable& other) const;
  inline std::optional<bool> operator<=(const somelib::ns::RenamedPartialComparable& other) const;
  inline std::optional<bool> operator>=(const somelib::ns::RenamedPartialComparable& other) const;
  inline std::optional<bool> operator<(const somelib::ns::RenamedPartialComparable& other) const;
  inline std::optional<bool> operator>(const somelib::ns::RenamedPartialComparable& other) const;

  inline somelib::diplomat::Optional<int8_t> test_nonstd(const somelib::ns::RenamedPartialComparable& other) const;

};

} // namespace
#endif // SOMELIB_ns_RenamedPartialComparable_D_HPP
