#ifndef SOMELIB_ns_RenamedMyIterable_D_HPP
#define SOMELIB_ns_RenamedMyIterable_D_HPP

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
namespace capi { struct RenamedMyIterable; }
class RenamedMyIterable;
namespace capi { struct RenamedMyIterator; }
class RenamedMyIterator;
} // namespace ns
} // namespace somelib



namespace somelib::ns {
namespace capi {
    struct RenamedMyIterable;
    extern "C" {
    void namespace_MyIterable_destroy(RenamedMyIterable* self);
    }
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedMyIterable;
using RenamedMyIterableRef = somelib::diplomat::Ref<RenamedMyIterable, const somelib::ns::capi::RenamedMyIterable>;
using RenamedMyIterableRefMut = somelib::diplomat::Ref<RenamedMyIterable, somelib::ns::capi::RenamedMyIterable>;

class RenamedMyIterable : public somelib::diplomat::OpaquePointer<RenamedMyIterable, somelib::ns::capi::RenamedMyIterable, somelib::ns::capi::namespace_MyIterable_destroy> {
public:

  inline static somelib::ns::RenamedMyIterable new_(somelib::diplomat::span<const uint8_t> x);

  inline somelib::ns::RenamedMyIterator iter() const DIPLOMAT_LIFETIME_BOUND;
  inline somelib::diplomat::next_to_iter_helper<somelib::ns::RenamedMyIterator> begin() const;
  inline std::nullopt_t end() const { return std::nullopt; }

  inline size_t __len__() const;

};

} // namespace
#endif // SOMELIB_ns_RenamedMyIterable_D_HPP
