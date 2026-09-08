#ifndef SOMELIB_MutableCallbackHolder_HPP
#define SOMELIB_MutableCallbackHolder_HPP

#include "MutableCallbackHolder.d.hpp"

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
    typedef struct DiplomatCallback_MutableCallbackHolder_new_fallible_func_result {union {int32_t ok; somelib::capi::FFIError err;}; bool is_ok;} DiplomatCallback_MutableCallbackHolder_new_fallible_func_result;

    typedef struct DiplomatCallback_MutableCallbackHolder_new_fallible_func {
        const void* data;
        DiplomatCallback_MutableCallbackHolder_new_fallible_func_result (*run_callback)(const void*, int32_t );
        void (*destructor)(const void*);
    } DiplomatCallback_MutableCallbackHolder_new_fallible_func;

    somelib::capi::MutableCallbackHolder* MutableCallbackHolder_new_fallible(DiplomatCallback_MutableCallbackHolder_new_fallible_func func_cb_wrap);

    int32_t MutableCallbackHolder_call(somelib::capi::MutableCallbackHolder* self, int32_t a);

    void MutableCallbackHolder_destroy(MutableCallbackHolder* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<somelib::MutableCallbackHolder> somelib::MutableCallbackHolder::new_fallible(std::function<somelib::diplomat::result<int32_t, somelib::FFIError>(int32_t)> func) {
    auto result = somelib::capi::MutableCallbackHolder_new_fallible({new decltype(func)(std::move(func)), somelib::diplomat::fn_traits(func).template c_run_callback_result<int32_t, somelib::FFIError, somelib::capi::DiplomatCallback_MutableCallbackHolder_new_fallible_func_result>, somelib::diplomat::fn_traits(func).c_delete});
    return std::unique_ptr<somelib::MutableCallbackHolder>(somelib::MutableCallbackHolder::FromFFI(result));
}

inline int32_t somelib::MutableCallbackHolder::call(int32_t a) {
    auto result = somelib::capi::MutableCallbackHolder_call(this->AsFFI(),
        a);
    return result;
}

inline const somelib::capi::MutableCallbackHolder* somelib::MutableCallbackHolder::AsFFI() const {
    return reinterpret_cast<const somelib::capi::MutableCallbackHolder*>(this);
}

inline somelib::capi::MutableCallbackHolder* somelib::MutableCallbackHolder::AsFFI() {
    return reinterpret_cast<somelib::capi::MutableCallbackHolder*>(this);
}

inline const somelib::MutableCallbackHolder* somelib::MutableCallbackHolder::FromFFI(const somelib::capi::MutableCallbackHolder* ptr) {
    return reinterpret_cast<const somelib::MutableCallbackHolder*>(ptr);
}

inline somelib::MutableCallbackHolder* somelib::MutableCallbackHolder::FromFFI(somelib::capi::MutableCallbackHolder* ptr) {
    return reinterpret_cast<somelib::MutableCallbackHolder*>(ptr);
}

inline void somelib::MutableCallbackHolder::operator delete(void* ptr) {
    somelib::capi::MutableCallbackHolder_destroy(reinterpret_cast<somelib::capi::MutableCallbackHolder*>(ptr));
}


#endif // SOMELIB_MutableCallbackHolder_HPP
