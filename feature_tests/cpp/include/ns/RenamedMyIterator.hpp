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

inline std::optional<uint8_t> somelib::ns::RenamedMyIterator::next() {
    auto result = somelib::ns::capi::namespace_MyIterator_next(this->AsFFI());
    return result.is_ok ? std::optional<uint8_t>(result.ok) : std::nullopt;
}

inline const somelib::ns::capi::RenamedMyIterator* somelib::ns::RenamedMyIterator::AsFFI() const {
    return reinterpret_cast<const somelib::ns::capi::RenamedMyIterator*>(this);
}

inline somelib::ns::capi::RenamedMyIterator* somelib::ns::RenamedMyIterator::AsFFI() {
    return reinterpret_cast<somelib::ns::capi::RenamedMyIterator*>(this);
}

inline const somelib::ns::RenamedMyIterator* somelib::ns::RenamedMyIterator::FromFFI(const somelib::ns::capi::RenamedMyIterator* ptr) {
    return reinterpret_cast<const somelib::ns::RenamedMyIterator*>(ptr);
}

inline somelib::ns::RenamedMyIterator* somelib::ns::RenamedMyIterator::FromFFI(somelib::ns::capi::RenamedMyIterator* ptr) {
    return reinterpret_cast<somelib::ns::RenamedMyIterator*>(ptr);
}

inline void somelib::ns::RenamedMyIterator::operator delete(void* ptr) {
    somelib::ns::capi::namespace_MyIterator_destroy(reinterpret_cast<somelib::ns::capi::RenamedMyIterator*>(ptr));
}


#endif // SOMELIB_ns_RenamedMyIterator_HPP
