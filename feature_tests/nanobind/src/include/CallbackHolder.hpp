#ifndef SOMELIB_CallbackHolder_HPP
#define SOMELIB_CallbackHolder_HPP

#include "CallbackHolder.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "FFIError.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {
    typedef struct DiplomatCallback_CallbackHolder_new_fallible_func_result {union {int32_t ok; somelib::capi::FFIError err;}; bool is_ok;} DiplomatCallback_CallbackHolder_new_fallible_func_result;

    typedef struct DiplomatCallback_CallbackHolder_new_fallible_func {
        const void* data;
        DiplomatCallback_CallbackHolder_new_fallible_func_result (*run_callback)(const void*, int32_t );
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackHolder_new_fallible_func;

    somelib::capi::CallbackHolder* CallbackHolder_new_fallible(DiplomatCallback_CallbackHolder_new_fallible_func func_cb_wrap);

    int32_t CallbackHolder_call(const somelib::capi::CallbackHolder* self, int32_t a);

    void CallbackHolder_destroy(CallbackHolder* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<somelib::CallbackHolder> somelib::CallbackHolder::new_fallible(std::function<somelib::diplomat::result<int32_t, somelib::FFIError>(int32_t)> func) {
    auto result = somelib::capi::CallbackHolder_new_fallible({new decltype(func)(std::move(func)), somelib::diplomat::fn_traits(func).template c_run_callback_result<int32_t, somelib::FFIError, somelib::capi::DiplomatCallback_CallbackHolder_new_fallible_func_result>, somelib::diplomat::fn_traits(func).c_delete});
    return std::unique_ptr<somelib::CallbackHolder>(somelib::CallbackHolder::FromFFI(result));
}

inline int32_t somelib::CallbackHolder::call(int32_t a) const {
    auto result = somelib::capi::CallbackHolder_call(this->AsFFI(),
        a);
    return result;
}

inline const somelib::capi::CallbackHolder* somelib::CallbackHolder::AsFFI() const {
    return reinterpret_cast<const somelib::capi::CallbackHolder*>(this);
}

inline somelib::capi::CallbackHolder* somelib::CallbackHolder::AsFFI() {
    return reinterpret_cast<somelib::capi::CallbackHolder*>(this);
}

inline const somelib::CallbackHolder* somelib::CallbackHolder::FromFFI(const somelib::capi::CallbackHolder* ptr) {
    return reinterpret_cast<const somelib::CallbackHolder*>(ptr);
}

inline somelib::CallbackHolder* somelib::CallbackHolder::FromFFI(somelib::capi::CallbackHolder* ptr) {
    return reinterpret_cast<somelib::CallbackHolder*>(ptr);
}

inline void somelib::CallbackHolder::operator delete(void* ptr) {
    somelib::capi::CallbackHolder_destroy(reinterpret_cast<somelib::capi::CallbackHolder*>(ptr));
}


#endif // SOMELIB_CallbackHolder_HPP
