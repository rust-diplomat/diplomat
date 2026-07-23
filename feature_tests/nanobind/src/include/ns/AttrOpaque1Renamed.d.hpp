#ifndef SOMELIB_ns_AttrOpaque1Renamed_D_HPP
#define SOMELIB_ns_AttrOpaque1Renamed_D_HPP

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
namespace capi { struct Unnamespaced; }
class Unnamespaced;
} // namespace somelib
namespace somelib {
namespace ns {
namespace capi { struct AttrOpaque1Renamed; }
class AttrOpaque1Renamed;
class RenamedAttrEnum;
} // namespace ns
} // namespace somelib



namespace somelib::ns {
namespace capi {
    struct AttrOpaque1Renamed;
    extern "C" {
    void namespace_AttrOpaque1_destroy(AttrOpaque1Renamed* self);
    }
} // namespace capi
} // namespace

namespace somelib::ns {
class AttrOpaque1Renamed;
using AttrOpaque1RenamedRef = somelib::diplomat::Ref<AttrOpaque1Renamed, const somelib::ns::capi::AttrOpaque1Renamed>;
using AttrOpaque1RenamedRefMut = somelib::diplomat::Ref<AttrOpaque1Renamed, somelib::ns::capi::AttrOpaque1Renamed>;

/**
 * Some example docs
 * Some Nanobind/C++ example docs
 * Back to all docs
 */
class AttrOpaque1Renamed : public somelib::diplomat::OpaquePointer<AttrOpaque1Renamed, somelib::ns::capi::AttrOpaque1Renamed, somelib::ns::capi::namespace_AttrOpaque1_destroy> {
public:

  inline static somelib::ns::AttrOpaque1Renamed new_overload(int32_t _i);

  /**
   * More example docs
   */
  inline static somelib::ns::AttrOpaque1Renamed totally_not_new();

  inline static void test_namespaced_callback(std::function<somelib::diplomat::result<std::monostate, std::monostate>()> _t);

  inline static int32_t mac_test();

  inline static int32_t hello();

  inline uint8_t method_renamed() const;

  inline uint8_t abirenamed() const;

  inline void use_unnamespaced(const somelib::Unnamespaced& _un) const;

  inline void use_namespaced(somelib::ns::RenamedAttrEnum _n) const;

};

} // namespace
#endif // SOMELIB_ns_AttrOpaque1Renamed_D_HPP
