#ifndef SOMELIB_ns_RenamedStringList_HPP
#define SOMELIB_ns_RenamedStringList_HPP

#include "RenamedStringList.d.hpp"

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

    somelib::ns::capi::RenamedStringList* namespace_StringList_return_new(void);

    void namespace_StringList_destroy(RenamedStringList* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<somelib::ns::RenamedStringList> somelib::ns::RenamedStringList::return_new() {
    auto result = somelib::ns::capi::namespace_StringList_return_new();
    return std::unique_ptr<somelib::ns::RenamedStringList>(somelib::ns::RenamedStringList::FromFFI(result));
}

inline const somelib::ns::capi::RenamedStringList* somelib::ns::RenamedStringList::AsFFI() const {
    return reinterpret_cast<const somelib::ns::capi::RenamedStringList*>(this);
}

inline somelib::ns::capi::RenamedStringList* somelib::ns::RenamedStringList::AsFFI() {
    return reinterpret_cast<somelib::ns::capi::RenamedStringList*>(this);
}

inline const somelib::ns::RenamedStringList* somelib::ns::RenamedStringList::FromFFI(const somelib::ns::capi::RenamedStringList* ptr) {
    return reinterpret_cast<const somelib::ns::RenamedStringList*>(ptr);
}

inline somelib::ns::RenamedStringList* somelib::ns::RenamedStringList::FromFFI(somelib::ns::capi::RenamedStringList* ptr) {
    return reinterpret_cast<somelib::ns::RenamedStringList*>(ptr);
}

inline void somelib::ns::RenamedStringList::operator delete(void* ptr) {
    somelib::ns::capi::namespace_StringList_destroy(reinterpret_cast<somelib::ns::capi::RenamedStringList*>(ptr));
}

namespace nanobind::detail {
    template<>
    struct type_caster<std::unique_ptr<somelib::ns::RenamedStringList>>
    {
        Py_ssize_t size;
        using Caster = list_caster<std::vector<std::string>, std::string>;
        static constexpr auto Name = Caster::Name;

        NB_INLINE bool can_cast() const noexcept { return true; }

        static handle from_cpp(std::unique_ptr<somelib::ns::RenamedStringList> value, rv_policy p, cleanup_list* cl) noexcept {
            somelib::ns::RenamedStringList* val = value.release();
            auto ptr = (somelib::diplomat::capi::DiplomatStringView*) val;
            std::string test = std::string(ptr->data, ptr->len);

            somelib::ns::capi::namespace_StringList_destroy((somelib::ns::capi::RenamedStringList*)val);

            std::vector<std::string> vec = {test};
            for (char c : test) {
                vec.push_back(std::string{c});
            }
            return Caster::from_cpp(vec, p, cl);
        }

        bool from_python(handle src, uint8_t flags, cleanup_list* cl) noexcept {
            return false;
        }
    };
}

#endif // SOMELIB_ns_RenamedStringList_HPP
