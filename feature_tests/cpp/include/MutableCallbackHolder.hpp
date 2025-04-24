#ifndef MutableCallbackHolder_HPP
#define MutableCallbackHolder_HPP

#include "MutableCallbackHolder.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    typedef struct DiplomatCallback_MutableCallbackHolder_new_func {
        const void* data;
        int32_t (*run_callback)(const void*, int32_t );
        void (*destructor)(const void*);
    } DiplomatCallback_MutableCallbackHolder_new_func;
    
    diplomat::capi::MutableCallbackHolder* MutableCallbackHolder_new(DiplomatCallback_MutableCallbackHolder_new_func func_cb_wrap);
    
    int32_t MutableCallbackHolder_call(diplomat::capi::MutableCallbackHolder* self, int32_t a);
    
    
    void MutableCallbackHolder_destroy(MutableCallbackHolder* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<MutableCallbackHolder> MutableCallbackHolder::new_(std::function<int32_t(int32_t)> func) {
  auto result = diplomat::capi::MutableCallbackHolder_new({new decltype(func)(std::move(func)), diplomat::fn_traits(func).c_run_callback, diplomat::fn_traits(func).c_delete});
  return std::unique_ptr<MutableCallbackHolder>(MutableCallbackHolder::FromFFI(result));
}

inline int32_t MutableCallbackHolder::call(int32_t a) {
  auto result = diplomat::capi::MutableCallbackHolder_call(this->AsFFI(),
    a);
  return result;
}

inline const diplomat::capi::MutableCallbackHolder* MutableCallbackHolder::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::MutableCallbackHolder*>(this);
}

inline diplomat::capi::MutableCallbackHolder* MutableCallbackHolder::AsFFI() {
  return reinterpret_cast<diplomat::capi::MutableCallbackHolder*>(this);
}

inline const MutableCallbackHolder* MutableCallbackHolder::FromFFI(const diplomat::capi::MutableCallbackHolder* ptr) {
  return reinterpret_cast<const MutableCallbackHolder*>(ptr);
}

inline MutableCallbackHolder* MutableCallbackHolder::FromFFI(diplomat::capi::MutableCallbackHolder* ptr) {
  return reinterpret_cast<MutableCallbackHolder*>(ptr);
}

inline void MutableCallbackHolder::operator delete(void* ptr) {
  diplomat::capi::MutableCallbackHolder_destroy(reinterpret_cast<diplomat::capi::MutableCallbackHolder*>(ptr));
}


#endif // MutableCallbackHolder_HPP
