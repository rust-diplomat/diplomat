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
#include "FFIError.hpp"
#include "MyString.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {
    typedef struct DiplomatCallback_OpaqueCallbacks_ret_op_fallible_f_result {union {const somelib::capi::MyString* ok; somelib::capi::FFIError err;}; bool is_ok;} DiplomatCallback_OpaqueCallbacks_ret_op_fallible_f_result;

    typedef struct DiplomatCallback_OpaqueCallbacks_ret_op_fallible_f {
        const void* data;
        DiplomatCallback_OpaqueCallbacks_ret_op_fallible_f_result (*run_callback)(const void*, const somelib::capi::MyString* );
        void (*destructor)(const void*);
    } DiplomatCallback_OpaqueCallbacks_ret_op_fallible_f;
    typedef struct DiplomatCallback_OpaqueCallbacks_ctor_fallible_f_result {union {const somelib::capi::MyString* ok; somelib::capi::FFIError err;}; bool is_ok;} DiplomatCallback_OpaqueCallbacks_ctor_fallible_f_result;

    typedef struct DiplomatCallback_OpaqueCallbacks_ctor_fallible_f {
        const void* data;
        DiplomatCallback_OpaqueCallbacks_ctor_fallible_f_result (*run_callback)(const void*, const somelib::capi::MyString* );
        void (*destructor)(const void*);
    } DiplomatCallback_OpaqueCallbacks_ctor_fallible_f;
    typedef struct DiplomatCallback_OpaqueCallbacks_opaque_cb_self_fallible_cb_result {union {const somelib::capi::MyString* ok; somelib::capi::FFIError err;}; bool is_ok;} DiplomatCallback_OpaqueCallbacks_opaque_cb_self_fallible_cb_result;

    typedef struct DiplomatCallback_OpaqueCallbacks_opaque_cb_self_fallible_cb {
        const void* data;
        DiplomatCallback_OpaqueCallbacks_opaque_cb_self_fallible_cb_result (*run_callback)(const void*, const somelib::capi::MyString* );
        void (*destructor)(const void*);
    } DiplomatCallback_OpaqueCallbacks_opaque_cb_self_fallible_cb;
    typedef struct DiplomatCallback_OpaqueCallbacks_opaque_cb_mut_self_fallible_cb_result {union {const somelib::capi::MyString* ok; somelib::capi::FFIError err;}; bool is_ok;} DiplomatCallback_OpaqueCallbacks_opaque_cb_mut_self_fallible_cb_result;

    typedef struct DiplomatCallback_OpaqueCallbacks_opaque_cb_mut_self_fallible_cb {
        const void* data;
        DiplomatCallback_OpaqueCallbacks_opaque_cb_mut_self_fallible_cb_result (*run_callback)(const void*, const somelib::capi::MyString* );
        void (*destructor)(const void*);
    } DiplomatCallback_OpaqueCallbacks_opaque_cb_mut_self_fallible_cb;

    const somelib::capi::MyString* OpaqueCallbacks_ret_op_fallible(DiplomatCallback_OpaqueCallbacks_ret_op_fallible_f f_cb_wrap, const somelib::capi::MyString* st);

    somelib::capi::OpaqueCallbacks* OpaqueCallbacks_ctor_fallible(DiplomatCallback_OpaqueCallbacks_ctor_fallible_f f_cb_wrap, const somelib::capi::MyString* st);

    const somelib::capi::MyString* OpaqueCallbacks_opaque_cb_self_fallible(const somelib::capi::OpaqueCallbacks* self, DiplomatCallback_OpaqueCallbacks_opaque_cb_self_fallible_cb cb_cb_wrap, const somelib::capi::MyString* st);

    const somelib::capi::MyString* OpaqueCallbacks_opaque_cb_mut_self_fallible(somelib::capi::OpaqueCallbacks* self, DiplomatCallback_OpaqueCallbacks_opaque_cb_mut_self_fallible_cb cb_cb_wrap, const somelib::capi::MyString* st);

    void OpaqueCallbacks_destroy(OpaqueCallbacks* self);

    } // extern "C"
} // namespace capi
} // namespace

inline const somelib::MyString& somelib::OpaqueCallbacks::ret_op_fallible(std::function<somelib::diplomat::result<const somelib::MyString&, somelib::FFIError>(const somelib::MyString&)> f, const somelib::MyString& st) {
    auto result = somelib::capi::OpaqueCallbacks_ret_op_fallible({new decltype(f)(std::move(f)), somelib::diplomat::fn_traits(f).template c_run_callback_result<const somelib::MyString&, somelib::FFIError, somelib::capi::DiplomatCallback_OpaqueCallbacks_ret_op_fallible_f_result>, somelib::diplomat::fn_traits(f).c_delete},
        st.AsFFI());
    return *somelib::MyString::FromFFI(result);
}

inline std::unique_ptr<somelib::OpaqueCallbacks> somelib::OpaqueCallbacks::ctor_fallible(std::function<somelib::diplomat::result<const somelib::MyString&, somelib::FFIError>(const somelib::MyString&)> f, const somelib::MyString& st) {
    auto result = somelib::capi::OpaqueCallbacks_ctor_fallible({new decltype(f)(std::move(f)), somelib::diplomat::fn_traits(f).template c_run_callback_result<const somelib::MyString&, somelib::FFIError, somelib::capi::DiplomatCallback_OpaqueCallbacks_ctor_fallible_f_result>, somelib::diplomat::fn_traits(f).c_delete},
        st.AsFFI());
    return std::unique_ptr<somelib::OpaqueCallbacks>(somelib::OpaqueCallbacks::FromFFI(result));
}

inline const somelib::MyString& somelib::OpaqueCallbacks::opaque_cb_self_fallible(std::function<somelib::diplomat::result<const somelib::MyString&, somelib::FFIError>(const somelib::MyString&)> cb, const somelib::MyString& st) const {
    auto result = somelib::capi::OpaqueCallbacks_opaque_cb_self_fallible(this->AsFFI(),
        {new decltype(cb)(std::move(cb)), somelib::diplomat::fn_traits(cb).template c_run_callback_result<const somelib::MyString&, somelib::FFIError, somelib::capi::DiplomatCallback_OpaqueCallbacks_opaque_cb_self_fallible_cb_result>, somelib::diplomat::fn_traits(cb).c_delete},
        st.AsFFI());
    return *somelib::MyString::FromFFI(result);
}

inline const somelib::MyString& somelib::OpaqueCallbacks::opaque_cb_mut_self_fallible(std::function<somelib::diplomat::result<const somelib::MyString&, somelib::FFIError>(const somelib::MyString&)> cb, const somelib::MyString& st) {
    auto result = somelib::capi::OpaqueCallbacks_opaque_cb_mut_self_fallible(this->AsFFI(),
        {new decltype(cb)(std::move(cb)), somelib::diplomat::fn_traits(cb).template c_run_callback_result<const somelib::MyString&, somelib::FFIError, somelib::capi::DiplomatCallback_OpaqueCallbacks_opaque_cb_mut_self_fallible_cb_result>, somelib::diplomat::fn_traits(cb).c_delete},
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
