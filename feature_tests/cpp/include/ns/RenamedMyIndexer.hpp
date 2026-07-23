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

    somelib::ns::capi::RenamedMyIndexer* namespace_MyIndexer_new(somelib::diplomat::capi::DiplomatStringsView v);

    typedef struct namespace_MyIndexer_get_result {union {somelib::diplomat::capi::DiplomatStringView ok; }; bool is_ok;} namespace_MyIndexer_get_result;
    namespace_MyIndexer_get_result namespace_MyIndexer_get(const somelib::ns::capi::RenamedMyIndexer* self, size_t i);

    typedef struct namespace_MyIndexer_get_str_result {union {somelib::diplomat::capi::DiplomatStringView ok; }; bool is_ok;} namespace_MyIndexer_get_str_result;
    namespace_MyIndexer_get_str_result namespace_MyIndexer_get_str(const somelib::ns::capi::RenamedMyIndexer* self, somelib::diplomat::capi::DiplomatStringView s);

    void namespace_MyIndexer_destroy(RenamedMyIndexer* self);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::ns::RenamedMyIndexer somelib::ns::RenamedMyIndexer::new_(somelib::diplomat::span<const diplomat::string_view_for_slice> v) {
    auto result = somelib::ns::capi::namespace_MyIndexer_new({reinterpret_cast<const somelib::diplomat::capi::DiplomatStringView*>(v.data()), v.size()});
    return somelib::ns::RenamedMyIndexer::FromFFI(result);
}

inline somelib::diplomat::Optional<std::string_view> somelib::ns::RenamedMyIndexer::operator[](size_t i) const DIPLOMAT_LIFETIME_BOUND {
    auto result = somelib::ns::capi::namespace_MyIndexer_get(this->AsFFI(),
        i);
    return result.is_ok ? somelib::diplomat::Optional<std::string_view>(std::string_view(result.ok.data, result.ok.len)) : somelib::diplomat::Optional<std::string_view>(std::nullopt);
}

inline somelib::diplomat::Optional<std::string_view> somelib::ns::RenamedMyIndexer::operator[](std::string_view s) const DIPLOMAT_LIFETIME_BOUND {
    auto result = somelib::ns::capi::namespace_MyIndexer_get_str(this->AsFFI(),
        {s.data(), s.size()});
    return result.is_ok ? somelib::diplomat::Optional<std::string_view>(std::string_view(result.ok.data, result.ok.len)) : somelib::diplomat::Optional<std::string_view>(std::nullopt);
}


#endif // SOMELIB_ns_RenamedMyIndexer_HPP
