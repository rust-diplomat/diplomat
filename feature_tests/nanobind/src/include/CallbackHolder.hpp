#ifndef CallbackHolder_HPP
#define CallbackHolder_HPP

#include "CallbackHolder.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    typedef struct DiplomatCallback_CallbackHolder_new_func {
        const void* data;
        int32_t (*run_callback)(const void*, int32_t );
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackHolder_new_func;
    
    diplomat::capi::CallbackHolder* CallbackHolder_new(DiplomatCallback_CallbackHolder_new_func func_cb_wrap);
    
    int32_t CallbackHolder_call(const diplomat::capi::CallbackHolder* self, int32_t a);
    
    
    void CallbackHolder_destroy(CallbackHolder* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<CallbackHolder> CallbackHolder::new_(std::function<int32_t(int32_t)> func) {
  auto result = diplomat::capi::CallbackHolder_new({new decltype(func)(std::move(func)), diplomat::fn_traits(func).c_run_callback, diplomat::fn_traits(func).c_delete});
  return std::unique_ptr<CallbackHolder>(CallbackHolder::FromFFI(result));
}

inline int32_t CallbackHolder::call(int32_t a) const {
  auto result = diplomat::capi::CallbackHolder_call(this->AsFFI(),
    a);
  return result;
}

inline const diplomat::capi::CallbackHolder* CallbackHolder::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::CallbackHolder*>(this);
}

inline diplomat::capi::CallbackHolder* CallbackHolder::AsFFI() {
  return reinterpret_cast<diplomat::capi::CallbackHolder*>(this);
}

inline const CallbackHolder* CallbackHolder::FromFFI(const diplomat::capi::CallbackHolder* ptr) {
  return reinterpret_cast<const CallbackHolder*>(ptr);
}

inline CallbackHolder* CallbackHolder::FromFFI(diplomat::capi::CallbackHolder* ptr) {
  return reinterpret_cast<CallbackHolder*>(ptr);
}

inline void CallbackHolder::operator delete(void* ptr) {
  diplomat::capi::CallbackHolder_destroy(reinterpret_cast<diplomat::capi::CallbackHolder*>(ptr));
}


#endif // CallbackHolder_HPP
