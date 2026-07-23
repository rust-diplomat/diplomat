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

inline somelib::ns::RenamedOpaqueZSTIndexer somelib::ns::RenamedOpaqueZSTIndexer::new_() {
    auto result = somelib::ns::capi::namespace_OpaqueZSTIndexer_new();
    return somelib::ns::RenamedOpaqueZSTIndexer::FromFFI(result);
}

inline somelib::diplomat::Optional<somelib::ns::RenamedOpaqueZSTIndexer> somelib::ns::RenamedOpaqueZSTIndexer::operator[](size_t idx) const {
    auto result = somelib::ns::capi::namespace_OpaqueZSTIndexer_index(this->AsFFI(),
        idx);
    return somelib::diplomat::Optional<somelib::ns::RenamedOpaqueZSTIndexer>::FromFFI(result);
}


#endif // SOMELIB_ns_RenamedOpaqueZSTIndexer_HPP
