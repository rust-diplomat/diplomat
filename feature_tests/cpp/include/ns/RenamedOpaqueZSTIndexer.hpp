#ifndef SOMELIB_ns_RenamedOpaqueZSTIndexer_HPP
#define SOMELIB_ns_RenamedOpaqueZSTIndexer_HPP

#include "RenamedOpaqueZSTIndexer.d.hpp"

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

    somelib::ns::capi::RenamedOpaqueZSTIndexer* namespace_OpaqueZSTIndexer_new(void);

    somelib::ns::capi::RenamedOpaqueZSTIndexer* namespace_OpaqueZSTIndexer_index(const somelib::ns::capi::RenamedOpaqueZSTIndexer* self, size_t idx);

    void namespace_OpaqueZSTIndexer_destroy(RenamedOpaqueZSTIndexer* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<somelib::ns::RenamedOpaqueZSTIndexer> somelib::ns::RenamedOpaqueZSTIndexer::new_() {
    auto result = somelib::ns::capi::namespace_OpaqueZSTIndexer_new();
    return std::unique_ptr<somelib::ns::RenamedOpaqueZSTIndexer>(somelib::ns::RenamedOpaqueZSTIndexer::FromFFI(result));
}

inline std::unique_ptr<somelib::ns::RenamedOpaqueZSTIndexer> somelib::ns::RenamedOpaqueZSTIndexer::operator[](size_t idx) const {
    auto result = somelib::ns::capi::namespace_OpaqueZSTIndexer_index(this->AsFFI(),
        idx);
    return std::unique_ptr<somelib::ns::RenamedOpaqueZSTIndexer>(somelib::ns::RenamedOpaqueZSTIndexer::FromFFI(result));
}

inline const somelib::ns::capi::RenamedOpaqueZSTIndexer* somelib::ns::RenamedOpaqueZSTIndexer::AsFFI() const {
    return reinterpret_cast<const somelib::ns::capi::RenamedOpaqueZSTIndexer*>(this);
}

inline somelib::ns::capi::RenamedOpaqueZSTIndexer* somelib::ns::RenamedOpaqueZSTIndexer::AsFFI() {
    return reinterpret_cast<somelib::ns::capi::RenamedOpaqueZSTIndexer*>(this);
}

inline const somelib::ns::RenamedOpaqueZSTIndexer* somelib::ns::RenamedOpaqueZSTIndexer::FromFFI(const somelib::ns::capi::RenamedOpaqueZSTIndexer* ptr) {
    return reinterpret_cast<const somelib::ns::RenamedOpaqueZSTIndexer*>(ptr);
}

inline somelib::ns::RenamedOpaqueZSTIndexer* somelib::ns::RenamedOpaqueZSTIndexer::FromFFI(somelib::ns::capi::RenamedOpaqueZSTIndexer* ptr) {
    return reinterpret_cast<somelib::ns::RenamedOpaqueZSTIndexer*>(ptr);
}

inline void somelib::ns::RenamedOpaqueZSTIndexer::operator delete(void* ptr) {
    somelib::ns::capi::namespace_OpaqueZSTIndexer_destroy(reinterpret_cast<somelib::ns::capi::RenamedOpaqueZSTIndexer*>(ptr));
}


#endif // SOMELIB_ns_RenamedOpaqueZSTIndexer_HPP
