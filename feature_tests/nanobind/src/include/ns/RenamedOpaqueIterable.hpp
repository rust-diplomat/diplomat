#ifndef ns_RenamedOpaqueIterable_HPP
#define ns_RenamedOpaqueIterable_HPP

#include "RenamedOpaqueIterable.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
#include "RenamedOpaqueIterator.hpp"


namespace ns {
namespace capi {
    extern "C" {

    ns::capi::RenamedOpaqueIterable* namespace_OpaqueIterable_new(size_t size);

    ns::capi::RenamedOpaqueIterator* namespace_OpaqueIterable_iter(const ns::capi::RenamedOpaqueIterable* self);

    void namespace_OpaqueIterable_destroy(RenamedOpaqueIterable* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<ns::RenamedOpaqueIterable> ns::RenamedOpaqueIterable::new_(size_t size) {
    auto result = ns::capi::namespace_OpaqueIterable_new(size);
    return std::unique_ptr<ns::RenamedOpaqueIterable>(ns::RenamedOpaqueIterable::FromFFI(result));
}

inline std::unique_ptr<ns::RenamedOpaqueIterator> ns::RenamedOpaqueIterable::iter() const {
    auto result = ns::capi::namespace_OpaqueIterable_iter(this->AsFFI());
    return std::unique_ptr<ns::RenamedOpaqueIterator>(ns::RenamedOpaqueIterator::FromFFI(result));
}

inline diplomat::next_to_iter_helper<ns::RenamedOpaqueIterator>ns::RenamedOpaqueIterable::begin() const {
    return iter();
}

inline const ns::capi::RenamedOpaqueIterable* ns::RenamedOpaqueIterable::AsFFI() const {
    return reinterpret_cast<const ns::capi::RenamedOpaqueIterable*>(this);
}

inline ns::capi::RenamedOpaqueIterable* ns::RenamedOpaqueIterable::AsFFI() {
    return reinterpret_cast<ns::capi::RenamedOpaqueIterable*>(this);
}

inline const ns::RenamedOpaqueIterable* ns::RenamedOpaqueIterable::FromFFI(const ns::capi::RenamedOpaqueIterable* ptr) {
    return reinterpret_cast<const ns::RenamedOpaqueIterable*>(ptr);
}

inline ns::RenamedOpaqueIterable* ns::RenamedOpaqueIterable::FromFFI(ns::capi::RenamedOpaqueIterable* ptr) {
    return reinterpret_cast<ns::RenamedOpaqueIterable*>(ptr);
}

inline void ns::RenamedOpaqueIterable::operator delete(void* ptr) {
    ns::capi::namespace_OpaqueIterable_destroy(reinterpret_cast<ns::capi::RenamedOpaqueIterable*>(ptr));
}


#endif // ns_RenamedOpaqueIterable_HPP
