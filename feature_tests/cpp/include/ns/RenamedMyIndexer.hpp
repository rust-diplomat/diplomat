#ifndef SOMELIB_ns_RenamedMyIndexer_HPP
#define SOMELIB_ns_RenamedMyIndexer_HPP

#include "RenamedMyIndexer.d.hpp"

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

    typedef struct namespace_MyIndexer_get_result {union {somelib::diplomat::capi::DiplomatStringView ok; }; bool is_ok;} namespace_MyIndexer_get_result;
    namespace_MyIndexer_get_result namespace_MyIndexer_get(const somelib::ns::capi::RenamedMyIndexer* self, size_t i);

    void namespace_MyIndexer_destroy(RenamedMyIndexer* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::optional<std::string_view> somelib::ns::RenamedMyIndexer::operator[](size_t i) const {
    auto result = somelib::ns::capi::namespace_MyIndexer_get(this->AsFFI(),
        i);
    return result.is_ok ? std::optional<std::string_view>(std::string_view(result.ok.data, result.ok.len)) : std::nullopt;
}

inline const somelib::ns::capi::RenamedMyIndexer* somelib::ns::RenamedMyIndexer::AsFFI() const {
    return reinterpret_cast<const somelib::ns::capi::RenamedMyIndexer*>(this);
}

inline somelib::ns::capi::RenamedMyIndexer* somelib::ns::RenamedMyIndexer::AsFFI() {
    return reinterpret_cast<somelib::ns::capi::RenamedMyIndexer*>(this);
}

inline const somelib::ns::RenamedMyIndexer* somelib::ns::RenamedMyIndexer::FromFFI(const somelib::ns::capi::RenamedMyIndexer* ptr) {
    return reinterpret_cast<const somelib::ns::RenamedMyIndexer*>(ptr);
}

inline somelib::ns::RenamedMyIndexer* somelib::ns::RenamedMyIndexer::FromFFI(somelib::ns::capi::RenamedMyIndexer* ptr) {
    return reinterpret_cast<somelib::ns::RenamedMyIndexer*>(ptr);
}

inline void somelib::ns::RenamedMyIndexer::operator delete(void* ptr) {
    somelib::ns::capi::namespace_MyIndexer_destroy(reinterpret_cast<somelib::ns::capi::RenamedMyIndexer*>(ptr));
}


#endif // SOMELIB_ns_RenamedMyIndexer_HPP
