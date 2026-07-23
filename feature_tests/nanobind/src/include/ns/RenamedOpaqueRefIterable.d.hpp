#ifndef SOMELIB_ns_RenamedOpaqueRefIterable_D_HPP
#define SOMELIB_ns_RenamedOpaqueRefIterable_D_HPP

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
namespace capi { struct RenamedOpaqueRefIterable; }
class RenamedOpaqueRefIterable;
namespace capi { struct RenamedOpaqueRefIterator; }
class RenamedOpaqueRefIterator;
} // namespace ns
} // namespace somelib



namespace somelib::ns {
namespace capi {
    struct RenamedOpaqueRefIterable;
    extern "C" {
    void namespace_OpaqueRefIterable_destroy(RenamedOpaqueRefIterable* self);
    }
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedOpaqueRefIterable;
using RenamedOpaqueRefIterableRef = somelib::diplomat::Ref<RenamedOpaqueRefIterable, const somelib::ns::capi::RenamedOpaqueRefIterable>;
using RenamedOpaqueRefIterableRefMut = somelib::diplomat::Ref<RenamedOpaqueRefIterable, somelib::ns::capi::RenamedOpaqueRefIterable>;

class RenamedOpaqueRefIterable : public somelib::diplomat::OpaquePointer<RenamedOpaqueRefIterable, somelib::ns::capi::RenamedOpaqueRefIterable, somelib::ns::capi::namespace_OpaqueRefIterable_destroy> {
public:

  inline static somelib::ns::RenamedOpaqueRefIterable new_(size_t size);

  inline somelib::ns::RenamedOpaqueRefIterator iter() const DIPLOMAT_LIFETIME_BOUND;
  inline somelib::diplomat::next_to_iter_helper<somelib::ns::RenamedOpaqueRefIterator> begin() const;
  inline std::nullopt_t end() const { return std::nullopt; }

};

} // namespace
#endif // SOMELIB_ns_RenamedOpaqueRefIterable_D_HPP
