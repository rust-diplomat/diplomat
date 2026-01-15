#ifndef SOMELIB_STRING_LIST_HPP
#define SOMELIB_STRING_LIST_HPP

#include "../diplomat_runtime.hpp"
#include "RenamedStringList.d.hpp"

extern "C" {
    void namespace_StringList_destroy(somelib::ns::capi::RenamedStringList* self);
    
    somelib::ns::capi::RenamedStringList* namespace_StringList_return_new(void);
}

namespace somelib::ns {
    std::vector<std::string> RenamedStringList::return_new() {
        somelib::ns::capi::RenamedStringList* self = namespace_StringList_return_new();
        const std::string arr = *reinterpret_cast<std::string*>(self);
        namespace_StringList_destroy(self);
        return std::vector<std::string>({arr});
    }
}

#endif