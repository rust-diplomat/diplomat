#ifndef ns_RenamedOpaqueRefIterable_HPP
#define ns_RenamedOpaqueRefIterable_HPP

#include "RenamedOpaqueRefIterable.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
#include "RenamedOpaqueRefIterator.hpp"


namespace ns {
namespace capi {
    extern "C" {

    ns::capi::RenamedOpaqueRefIterable* namespace_OpaqueRefIterable_new(size_t size);

    ns::capi::RenamedOpaqueRefIterator* namespace_OpaqueRefIterable_iter(const ns::capi::RenamedOpaqueRefIterable* self);

    void namespace_OpaqueRefIterable_destroy(RenamedOpaqueRefIterable* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<ns::RenamedOpaqueRefIterable> ns::RenamedOpaqueRefIterable::new_(size_t size) {
    auto result = ns::capi::namespace_OpaqueRefIterable_new(size);
    return std::unique_ptr<ns::RenamedOpaqueRefIterable>(ns::RenamedOpaqueRefIterable::FromFFI(result));
}

inline std::unique_ptr<ns::RenamedOpaqueRefIterator> ns::RenamedOpaqueRefIterable::iter() const {
    auto result = ns::capi::namespace_OpaqueRefIterable_iter(this->AsFFI());
    return std::unique_ptr<ns::RenamedOpaqueRefIterator>(ns::RenamedOpaqueRefIterator::FromFFI(result));
}

inline diplomat::next_to_iter_helper<ns::RenamedOpaqueRefIterator>ns::RenamedOpaqueRefIterable::begin() const {
    return iter();
}

inline const ns::capi::RenamedOpaqueRefIterable* ns::RenamedOpaqueRefIterable::AsFFI() const {
    return reinterpret_cast<const ns::capi::RenamedOpaqueRefIterable*>(this);
}

inline ns::capi::RenamedOpaqueRefIterable* ns::RenamedOpaqueRefIterable::AsFFI() {
    return reinterpret_cast<ns::capi::RenamedOpaqueRefIterable*>(this);
}

inline const ns::RenamedOpaqueRefIterable* ns::RenamedOpaqueRefIterable::FromFFI(const ns::capi::RenamedOpaqueRefIterable* ptr) {
    return reinterpret_cast<const ns::RenamedOpaqueRefIterable*>(ptr);
}

inline ns::RenamedOpaqueRefIterable* ns::RenamedOpaqueRefIterable::FromFFI(ns::capi::RenamedOpaqueRefIterable* ptr) {
    return reinterpret_cast<ns::RenamedOpaqueRefIterable*>(ptr);
}

inline void ns::RenamedOpaqueRefIterable::operator delete(void* ptr) {
    ns::capi::namespace_OpaqueRefIterable_destroy(reinterpret_cast<ns::capi::RenamedOpaqueRefIterable*>(ptr));
}


#endif // ns_RenamedOpaqueRefIterable_HPP
