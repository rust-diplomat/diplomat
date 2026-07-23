#ifndef SOMELIB_ns_RenamedMyIterator_D_HPP
#define SOMELIB_ns_RenamedMyIterator_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"


namespace somelib::ns {
namespace capi {
    struct RenamedMyIterator;
    extern "C" {
    void namespace_MyIterator_destroy(RenamedMyIterator* self);
    }
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedMyIterator;
using RenamedMyIteratorRef = somelib::diplomat::Ref<RenamedMyIterator, const somelib::ns::capi::RenamedMyIterator>;
using RenamedMyIteratorRefMut = somelib::diplomat::Ref<RenamedMyIterator, somelib::ns::capi::RenamedMyIterator>;

class RenamedMyIterator : public somelib::diplomat::OpaquePointer<RenamedMyIterator, somelib::ns::capi::RenamedMyIterator, somelib::ns::capi::namespace_MyIterator_destroy> {
public:

  inline somelib::diplomat::Optional<uint8_t> next();

};

} // namespace
#endif // SOMELIB_ns_RenamedMyIterator_D_HPP
