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
    typedef struct DiplomatCallback_OpaqueCallbacks_ctor_f {
        const void* data;
        const somelib::capi::MyString* (*run_callback)(const void*, const somelib::capi::MyString* );
        void (*destructor)(const void*);
    } DiplomatCallback_OpaqueCallbacks_ctor_f;
    typedef struct DiplomatCallback_OpaqueCallbacks_opaque_cb_self_cb {
        const void* data;
        void (*run_callback)(const void*, const somelib::capi::MyString* );
        void (*destructor)(const void*);
    } DiplomatCallback_OpaqueCallbacks_opaque_cb_self_cb;
    typedef struct DiplomatCallback_OpaqueCallbacks_opaque_cb_mut_self_cb {
        const void* data;
        void (*run_callback)(const void*, const somelib::capi::MyString* );
        void (*destructor)(const void*);
    } DiplomatCallback_OpaqueCallbacks_opaque_cb_mut_self_cb;

    const somelib::capi::MyString* OpaqueCallbacks_ret_op(DiplomatCallback_OpaqueCallbacks_ret_op_f f_cb_wrap, const somelib::capi::MyString* st);

    somelib::capi::OpaqueCallbacks* OpaqueCallbacks_ctor(DiplomatCallback_OpaqueCallbacks_ctor_f f_cb_wrap, const somelib::capi::MyString* st);

    void OpaqueCallbacks_opaque_cb_self(const somelib::capi::OpaqueCallbacks* self, DiplomatCallback_OpaqueCallbacks_opaque_cb_self_cb cb_cb_wrap, const somelib::capi::MyString* st);

    void OpaqueCallbacks_opaque_cb_mut_self(somelib::capi::OpaqueCallbacks* self, DiplomatCallback_OpaqueCallbacks_opaque_cb_mut_self_cb cb_cb_wrap, const somelib::capi::MyString* st);

    void OpaqueCallbacks_destroy(OpaqueCallbacks* self);

    } // extern "C"
} // namespace capi
} // namespace

inline const somelib::MyString& somelib::OpaqueCallbacks::ret_op(std::function<const somelib::MyString&(const somelib::MyString&)> f, const somelib::MyString& st) {
    auto result = somelib::capi::OpaqueCallbacks_ret_op({new decltype(f)(std::move(f)), somelib::diplomat::fn_traits(f).template c_run_callback_diplomat_opaque<const somelib::capi::MyString*>, somelib::diplomat::fn_traits(f).c_delete},
        st.AsFFI());
    return *somelib::MyString::FromFFI(result);
}

inline std::unique_ptr<somelib::OpaqueCallbacks> somelib::OpaqueCallbacks::ctor(std::function<const somelib::MyString&(const somelib::MyString&)> f, const somelib::MyString& st) {
    auto result = somelib::capi::OpaqueCallbacks_ctor({new decltype(f)(std::move(f)), somelib::diplomat::fn_traits(f).template c_run_callback_diplomat_opaque<const somelib::capi::MyString*>, somelib::diplomat::fn_traits(f).c_delete},
        st.AsFFI());
    return std::unique_ptr<somelib::OpaqueCallbacks>(somelib::OpaqueCallbacks::FromFFI(result));
}

inline void somelib::OpaqueCallbacks::opaque_cb_self(std::function<void(const somelib::MyString&)> cb, const somelib::MyString& st) const {
    somelib::capi::OpaqueCallbacks_opaque_cb_self(this->AsFFI(),
        {new decltype(cb)(std::move(cb)), somelib::diplomat::fn_traits(cb).c_run_callback, somelib::diplomat::fn_traits(cb).c_delete},
        st.AsFFI());
}

inline void somelib::OpaqueCallbacks::opaque_cb_mut_self(std::function<void(const somelib::MyString&)> cb, const somelib::MyString& st) {
    somelib::capi::OpaqueCallbacks_opaque_cb_mut_self(this->AsFFI(),
        {new decltype(cb)(std::move(cb)), somelib::diplomat::fn_traits(cb).c_run_callback, somelib::diplomat::fn_traits(cb).c_delete},
        st.AsFFI());
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
