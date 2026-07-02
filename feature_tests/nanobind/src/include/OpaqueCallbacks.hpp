#ifndef SOMELIB_OpaqueCallbacks_HPP
#define SOMELIB_OpaqueCallbacks_HPP

#include "OpaqueCallbacks.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "MyString.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {
    typedef struct DiplomatCallback_OpaqueCallbacks_ret_op_f {
        const void* data;
        const somelib::capi::MyString* (*run_callback)(const void*, const somelib::capi::MyString* );
        void (*destructor)(const void*);
    } DiplomatCallback_OpaqueCallbacks_ret_op_f;

    const somelib::capi::MyString* OpaqueCallbacks_ret_op(DiplomatCallback_OpaqueCallbacks_ret_op_f f_cb_wrap, const somelib::capi::MyString* st);

    void OpaqueCallbacks_destroy(OpaqueCallbacks* self);

    } // extern "C"
} // namespace capi
} // namespace

inline const somelib::MyString& somelib::OpaqueCallbacks::ret_op(std::function<const somelib::MyString&(const somelib::MyString&)> f, const somelib::MyString& st) {
    auto result = somelib::capi::OpaqueCallbacks_ret_op({new decltype(f)(std::move(f)), somelib::diplomat::fn_traits(f).template c_run_callback_diplomat_opaque<const somelib::capi::MyString*>, somelib::diplomat::fn_traits(f).c_delete},
        st.AsFFI());
    return *somelib::MyString::FromFFI(result);
}

inline const somelib::capi::OpaqueCallbacks* somelib::OpaqueCallbacks::AsFFI() const {
    return reinterpret_cast<const somelib::capi::OpaqueCallbacks*>(this);
}

inline somelib::capi::OpaqueCallbacks* somelib::OpaqueCallbacks::AsFFI() {
    return reinterpret_cast<somelib::capi::OpaqueCallbacks*>(this);
}

inline const somelib::OpaqueCallbacks* somelib::OpaqueCallbacks::FromFFI(const somelib::capi::OpaqueCallbacks* ptr) {
    return reinterpret_cast<const somelib::OpaqueCallbacks*>(ptr);
}

inline somelib::OpaqueCallbacks* somelib::OpaqueCallbacks::FromFFI(somelib::capi::OpaqueCallbacks* ptr) {
    return reinterpret_cast<somelib::OpaqueCallbacks*>(ptr);
}

inline void somelib::OpaqueCallbacks::operator delete(void* ptr) {
    somelib::capi::OpaqueCallbacks_destroy(reinterpret_cast<somelib::capi::OpaqueCallbacks*>(ptr));
}


#endif // SOMELIB_OpaqueCallbacks_HPP
