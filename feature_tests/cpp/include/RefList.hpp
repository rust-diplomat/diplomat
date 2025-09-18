#ifndef SOMELIB_RefList_HPP
#define SOMELIB_RefList_HPP

#include "RefList.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "RefListParameter.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    somelib::capi::RefList* RefList_node(const somelib::capi::RefListParameter* data);

    void RefList_destroy(RefList* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<somelib::RefList> somelib::RefList::node(const somelib::RefListParameter& data) {
    auto result = somelib::capi::RefList_node(data.AsFFI());
    return std::unique_ptr<somelib::RefList>(somelib::RefList::FromFFI(result));
}

inline const somelib::capi::RefList* somelib::RefList::AsFFI() const {
    return reinterpret_cast<const somelib::capi::RefList*>(this);
}

inline somelib::capi::RefList* somelib::RefList::AsFFI() {
    return reinterpret_cast<somelib::capi::RefList*>(this);
}

inline const somelib::RefList* somelib::RefList::FromFFI(const somelib::capi::RefList* ptr) {
    return reinterpret_cast<const somelib::RefList*>(ptr);
}

inline somelib::RefList* somelib::RefList::FromFFI(somelib::capi::RefList* ptr) {
    return reinterpret_cast<somelib::RefList*>(ptr);
}

inline void somelib::RefList::operator delete(void* ptr) {
    somelib::capi::RefList_destroy(reinterpret_cast<somelib::capi::RefList*>(ptr));
}


#endif // SOMELIB_RefList_HPP
