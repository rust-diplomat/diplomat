#ifndef SOMELIB_ns_RenamedMyIterable_HPP
#define SOMELIB_ns_RenamedMyIterable_HPP

#include "RenamedMyIterable.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
#include "RenamedMyIterator.hpp"


namespace somelib::ns {
namespace capi {
    extern "C" {

    somelib::ns::capi::RenamedMyIterable* namespace_MyIterable_new(somelib::diplomat::capi::DiplomatU8View x);

    somelib::ns::capi::RenamedMyIterator* namespace_MyIterable_iter(const somelib::ns::capi::RenamedMyIterable* self);

    size_t namespace_MyIterable_len(const somelib::ns::capi::RenamedMyIterable* self);

    void namespace_MyIterable_destroy(RenamedMyIterable* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<somelib::ns::RenamedMyIterable> somelib::ns::RenamedMyIterable::new_(somelib::diplomat::span<const uint8_t> x) {
    auto result = somelib::ns::capi::namespace_MyIterable_new({x.data(), x.size()});
    return std::unique_ptr<somelib::ns::RenamedMyIterable>(somelib::ns::RenamedMyIterable::FromFFI(result));
}

inline std::unique_ptr<somelib::ns::RenamedMyIterator> somelib::ns::RenamedMyIterable::iter() const {
    auto result = somelib::ns::capi::namespace_MyIterable_iter(this->AsFFI());
    return std::unique_ptr<somelib::ns::RenamedMyIterator>(somelib::ns::RenamedMyIterator::FromFFI(result));
}

inline somelib::diplomat::next_to_iter_helper<somelib::ns::RenamedMyIterator>somelib::ns::RenamedMyIterable::begin() const {
    return iter();
}

inline size_t somelib::ns::RenamedMyIterable::__len__() const {
    auto result = somelib::ns::capi::namespace_MyIterable_len(this->AsFFI());
    return result;
}

inline const somelib::ns::capi::RenamedMyIterable* somelib::ns::RenamedMyIterable::AsFFI() const {
    return reinterpret_cast<const somelib::ns::capi::RenamedMyIterable*>(this);
}

inline somelib::ns::capi::RenamedMyIterable* somelib::ns::RenamedMyIterable::AsFFI() {
    return reinterpret_cast<somelib::ns::capi::RenamedMyIterable*>(this);
}

inline const somelib::ns::RenamedMyIterable* somelib::ns::RenamedMyIterable::FromFFI(const somelib::ns::capi::RenamedMyIterable* ptr) {
    return reinterpret_cast<const somelib::ns::RenamedMyIterable*>(ptr);
}

inline somelib::ns::RenamedMyIterable* somelib::ns::RenamedMyIterable::FromFFI(somelib::ns::capi::RenamedMyIterable* ptr) {
    return reinterpret_cast<somelib::ns::RenamedMyIterable*>(ptr);
}

inline void somelib::ns::RenamedMyIterable::operator delete(void* ptr) {
    somelib::ns::capi::namespace_MyIterable_destroy(reinterpret_cast<somelib::ns::capi::RenamedMyIterable*>(ptr));
}


#endif // SOMELIB_ns_RenamedMyIterable_HPP
