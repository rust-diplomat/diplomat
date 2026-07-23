#ifndef SOMELIB_ns_RenamedOpaqueIterable_D_HPP
#define SOMELIB_ns_RenamedOpaqueIterable_D_HPP

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
namespace capi { struct RenamedOpaqueIterable; }
class RenamedOpaqueIterable;
namespace capi { struct RenamedOpaqueIterator; }
class RenamedOpaqueIterator;
} // namespace ns
} // namespace somelib



namespace somelib::ns {
namespace capi {
    struct RenamedOpaqueIterable;
    extern "C" {
    void namespace_OpaqueIterable_destroy(RenamedOpaqueIterable* self);
    }
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedOpaqueIterable;
using RenamedOpaqueIterableRef = somelib::diplomat::Ref<RenamedOpaqueIterable, const somelib::ns::capi::RenamedOpaqueIterable>;
using RenamedOpaqueIterableRefMut = somelib::diplomat::Ref<RenamedOpaqueIterable, somelib::ns::capi::RenamedOpaqueIterable>;

class RenamedOpaqueIterable : public somelib::diplomat::OpaquePointer<RenamedOpaqueIterable, somelib::ns::capi::RenamedOpaqueIterable, somelib::ns::capi::namespace_OpaqueIterable_destroy> {
public:

  inline static somelib::ns::RenamedOpaqueIterable new_(size_t size);

  inline somelib::ns::RenamedOpaqueIterator iter() const DIPLOMAT_LIFETIME_BOUND;
  inline somelib::diplomat::next_to_iter_helper<somelib::ns::RenamedOpaqueIterator> begin() const;
  inline std::nullopt_t end() const { return std::nullopt; }

};

} // namespace
#endif // SOMELIB_ns_RenamedOpaqueIterable_D_HPP
