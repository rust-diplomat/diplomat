#ifndef SOMELIB_ns_RenamedOpaqueRefIterator_D_HPP
#define SOMELIB_ns_RenamedOpaqueRefIterator_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
#include "AttrOpaque1Renamed.d.hpp"
namespace somelib {
namespace ns {
namespace capi { struct AttrOpaque1Renamed; }
class AttrOpaque1Renamed;
} // namespace ns
} // namespace somelib



namespace somelib::ns {
namespace capi {
    struct RenamedOpaqueRefIterator;
    extern "C" {
    void namespace_OpaqueRefIterator_destroy(RenamedOpaqueRefIterator* self);
    }
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedOpaqueRefIterator;
using RenamedOpaqueRefIteratorRef = somelib::diplomat::Ref<RenamedOpaqueRefIterator, const somelib::ns::capi::RenamedOpaqueRefIterator>;
using RenamedOpaqueRefIteratorRefMut = somelib::diplomat::Ref<RenamedOpaqueRefIterator, somelib::ns::capi::RenamedOpaqueRefIterator>;

class RenamedOpaqueRefIterator : public somelib::diplomat::OpaquePointer<RenamedOpaqueRefIterator, somelib::ns::capi::RenamedOpaqueRefIterator, somelib::ns::capi::namespace_OpaqueRefIterator_destroy> {
public:

  inline somelib::diplomat::Optional<somelib::ns::AttrOpaque1RenamedRef> next() DIPLOMAT_LIFETIME_BOUND;

};

} // namespace
#endif // SOMELIB_ns_RenamedOpaqueRefIterator_D_HPP
