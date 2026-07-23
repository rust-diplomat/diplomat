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
namespace somelib {
namespace ns {
namespace capi { struct RenamedMyIndexer; }
class RenamedMyIndexer;
} // namespace ns
} // namespace somelib



namespace somelib::ns {
namespace capi {
    struct RenamedMyIndexer;
    extern "C" {
    void namespace_MyIndexer_destroy(RenamedMyIndexer* self);
    }
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedMyIndexer;
using RenamedMyIndexerRef = somelib::diplomat::Ref<RenamedMyIndexer, const somelib::ns::capi::RenamedMyIndexer>;
using RenamedMyIndexerRefMut = somelib::diplomat::Ref<RenamedMyIndexer, somelib::ns::capi::RenamedMyIndexer>;

class RenamedMyIndexer : public somelib::diplomat::OpaquePointer<RenamedMyIndexer, somelib::ns::capi::RenamedMyIndexer, somelib::ns::capi::namespace_MyIndexer_destroy> {
public:

  inline static somelib::ns::RenamedMyIndexer new_(somelib::diplomat::span<const diplomat::string_view_for_slice> v);

  inline somelib::diplomat::Optional<std::string_view> operator[](size_t i) const DIPLOMAT_LIFETIME_BOUND;

  inline somelib::diplomat::Optional<std::string_view> operator[](std::string_view s) const DIPLOMAT_LIFETIME_BOUND;

};

} // namespace
#endif // SOMELIB_ns_RenamedMyIndexer_D_HPP
