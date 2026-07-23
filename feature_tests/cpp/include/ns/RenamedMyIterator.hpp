#ifndef SOMELIB_ns_RenamedMyIterator_HPP
#define SOMELIB_ns_RenamedMyIterator_HPP

#include "RenamedMyIterator.d.hpp"

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
    extern "C" {

    typedef struct namespace_MyIterator_next_result {union {uint8_t ok; }; bool is_ok;} namespace_MyIterator_next_result;
    namespace_MyIterator_next_result namespace_MyIterator_next(somelib::ns::capi::RenamedMyIterator* self);

    void namespace_MyIterator_destroy(RenamedMyIterator* self);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::diplomat::Optional<uint8_t> somelib::ns::RenamedMyIterator::next() {
    auto result = somelib::ns::capi::namespace_MyIterator_next(this->AsFFI());
    return result.is_ok ? somelib::diplomat::Optional<uint8_t>(result.ok) : somelib::diplomat::Optional<uint8_t>(std::nullopt);
}


#endif // SOMELIB_ns_RenamedMyIterator_HPP
