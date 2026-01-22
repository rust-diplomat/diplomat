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

    void namespace_StringList_destroy(RenamedStringList* self);

    } // extern "C"
} // namespace capi
} // namespace

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

extern "C" {
    void namespace_StringList_destroy(somelib::ns::capi::RenamedStringList* self);

    somelib::ns::capi::RenamedStringList* namespace_StringList_return_new(void);
}

namespace somelib::ns {
    std::vector<std::string> RenamedStringList::return_new() {
        somelib::ns::capi::RenamedStringList* self = namespace_StringList_return_new();

        auto ptr = (somelib::diplomat::capi::DiplomatStringView*) self;
        std::string arr = std::string(ptr->data, ptr->len);

        namespace_StringList_destroy(self);

        return std::vector<std::string>({arr});
    }
}

#endif // SOMELIB_ns_RenamedStringList_HPP
