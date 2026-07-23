#ifndef SOMELIB_ns_RenamedOpaqueIterator_D_HPP
#define SOMELIB_ns_RenamedOpaqueIterator_D_HPP

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
namespace capi { struct AttrOpaque1Renamed; }
class AttrOpaque1Renamed;
} // namespace ns
} // namespace somelib



namespace somelib::ns {
namespace capi {
    struct RenamedOpaqueIterator;
    extern "C" {
    void namespace_OpaqueIterator_destroy(RenamedOpaqueIterator* self);
    }
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedOpaqueIterator;
using RenamedOpaqueIteratorRef = somelib::diplomat::Ref<RenamedOpaqueIterator, const somelib::ns::capi::RenamedOpaqueIterator>;
using RenamedOpaqueIteratorRefMut = somelib::diplomat::Ref<RenamedOpaqueIterator, somelib::ns::capi::RenamedOpaqueIterator>;

class RenamedOpaqueIterator : public somelib::diplomat::OpaquePointer<RenamedOpaqueIterator, somelib::ns::capi::RenamedOpaqueIterator, somelib::ns::capi::namespace_OpaqueIterator_destroy> {
public:

  inline somelib::diplomat::Optional<somelib::ns::AttrOpaque1Renamed> next();

};

} // namespace
#endif // SOMELIB_ns_RenamedOpaqueIterator_D_HPP
