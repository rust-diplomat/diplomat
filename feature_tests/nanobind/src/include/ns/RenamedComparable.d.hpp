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
    extern "C" {
    void namespace_Comparable_destroy(RenamedComparable* self);
    }
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedComparable;
using RenamedComparableRef = somelib::diplomat::Ref<RenamedComparable, const somelib::ns::capi::RenamedComparable>;
using RenamedComparableRefMut = somelib::diplomat::Ref<RenamedComparable, somelib::ns::capi::RenamedComparable>;

class RenamedComparable : public somelib::diplomat::OpaquePointer<RenamedComparable, somelib::ns::capi::RenamedComparable, somelib::ns::capi::namespace_Comparable_destroy> {
public:

  inline static somelib::ns::RenamedComparable new_(uint8_t int_);

  inline int8_t cmp(const somelib::ns::RenamedComparable& other) const;

  inline bool operator==(const somelib::ns::RenamedComparable& other) const;
  inline bool operator!=(const somelib::ns::RenamedComparable& other) const;
  inline bool operator<=(const somelib::ns::RenamedComparable& other) const;
  inline bool operator>=(const somelib::ns::RenamedComparable& other) const;
  inline bool operator<(const somelib::ns::RenamedComparable& other) const;
  inline bool operator>(const somelib::ns::RenamedComparable& other) const;

};

} // namespace
#endif // SOMELIB_ns_RenamedComparable_D_HPP
