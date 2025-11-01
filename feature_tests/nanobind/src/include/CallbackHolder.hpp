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
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {
    typedef struct DiplomatCallback_CallbackHolder_new_func {
        const void* data;
        int32_t (*run_callback)(const void*, int32_t );
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackHolder_new_func;

    somelib::capi::CallbackHolder* CallbackHolder_new(DiplomatCallback_CallbackHolder_new_func func_cb_wrap);

    int32_t CallbackHolder_call(const somelib::capi::CallbackHolder* self, int32_t a);

    void CallbackHolder_destroy(CallbackHolder* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<somelib::CallbackHolder> somelib::CallbackHolder::new_(std::function<int32_t(int32_t)> func) {
    auto result = somelib::capi::CallbackHolder_new({new decltype(func)(std::move(func)), somelib::diplomat::fn_traits(func).c_run_callback, somelib::diplomat::fn_traits(func).c_delete});
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
