#ifndef SOMELIB_ns_RenamedOpaqueZSTIndexer_D_HPP
#define SOMELIB_ns_RenamedOpaqueZSTIndexer_D_HPP

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
namespace capi { struct RenamedOpaqueZSTIndexer; }
class RenamedOpaqueZSTIndexer;
} // namespace ns
} // namespace somelib



namespace somelib::ns {
namespace capi {
    struct RenamedOpaqueZSTIndexer;
    extern "C" {
    void namespace_OpaqueZSTIndexer_destroy(RenamedOpaqueZSTIndexer* self);
    }
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedOpaqueZSTIndexer;
using RenamedOpaqueZSTIndexerRef = somelib::diplomat::Ref<RenamedOpaqueZSTIndexer, const somelib::ns::capi::RenamedOpaqueZSTIndexer>;
using RenamedOpaqueZSTIndexerRefMut = somelib::diplomat::Ref<RenamedOpaqueZSTIndexer, somelib::ns::capi::RenamedOpaqueZSTIndexer>;

class RenamedOpaqueZSTIndexer : public somelib::diplomat::OpaquePointer<RenamedOpaqueZSTIndexer, somelib::ns::capi::RenamedOpaqueZSTIndexer, somelib::ns::capi::namespace_OpaqueZSTIndexer_destroy> {
public:

  inline static somelib::ns::RenamedOpaqueZSTIndexer new_();

  inline somelib::diplomat::Optional<somelib::ns::RenamedOpaqueZSTIndexer> operator[](size_t idx) const;

};

} // namespace
#endif // SOMELIB_ns_RenamedOpaqueZSTIndexer_D_HPP
