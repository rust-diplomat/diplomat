#ifndef CallbackWrapper_HPP
#define CallbackWrapper_HPP

#include "CallbackWrapper.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "CallbackTestingStruct.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    typedef struct DiplomatCallback_CallbackWrapper_test_multi_arg_callback_f {
        const void* data;
        int32_t (*run_callback)(const void*, int32_t );
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_multi_arg_callback_f;
    typedef struct DiplomatCallback_CallbackWrapper_test_no_args_h {
        const void* data;
        void (*run_callback)(const void*);
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_no_args_h;
    typedef struct DiplomatCallback_CallbackWrapper_test_cb_with_struct_f {
        const void* data;
        int32_t (*run_callback)(const void*, diplomat::capi::CallbackTestingStruct );
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_cb_with_struct_f;
    typedef struct DiplomatCallback_CallbackWrapper_test_multiple_cb_args_f {
        const void* data;
        int32_t (*run_callback)(const void*);
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_multiple_cb_args_f;
    typedef struct DiplomatCallback_CallbackWrapper_test_multiple_cb_args_g {
        const void* data;
        int32_t (*run_callback)(const void*, int32_t );
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_multiple_cb_args_g;
    typedef struct DiplomatCallback_CallbackWrapper_test_str_cb_arg_f {
        const void* data;
        int32_t (*run_callback)(const void*, diplomat::capi::DiplomatStringView );
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_str_cb_arg_f;
    
    int32_t CallbackWrapper_test_multi_arg_callback(DiplomatCallback_CallbackWrapper_test_multi_arg_callback_f f_cb_wrap, int32_t x);
    
    int32_t CallbackWrapper_test_no_args(DiplomatCallback_CallbackWrapper_test_no_args_h h_cb_wrap);
    
    int32_t CallbackWrapper_test_cb_with_struct(DiplomatCallback_CallbackWrapper_test_cb_with_struct_f f_cb_wrap);
    
    int32_t CallbackWrapper_test_multiple_cb_args(DiplomatCallback_CallbackWrapper_test_multiple_cb_args_f f_cb_wrap, DiplomatCallback_CallbackWrapper_test_multiple_cb_args_g g_cb_wrap);
    
    int32_t CallbackWrapper_test_str_cb_arg(DiplomatCallback_CallbackWrapper_test_str_cb_arg_f f_cb_wrap);
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline int32_t CallbackWrapper::test_multi_arg_callback(std::function<int32_t(int32_t)> f, int32_t x) {
  auto result = diplomat::capi::CallbackWrapper_test_multi_arg_callback({new decltype(f)(f), diplomat::fn_traits(f).c_run_callback, diplomat::fn_traits(f).c_delete},
    x);
  return result;
}

inline int32_t CallbackWrapper::test_no_args(std::function<void()> h) {
  auto result = diplomat::capi::CallbackWrapper_test_no_args({new decltype(h)(h), diplomat::fn_traits(h).c_run_callback, diplomat::fn_traits(h).c_delete});
  return result;
}

inline int32_t CallbackWrapper::test_cb_with_struct(std::function<int32_t(CallbackTestingStruct)> f) {
  auto result = diplomat::capi::CallbackWrapper_test_cb_with_struct({new decltype(f)(f), diplomat::fn_traits(f).c_run_callback, diplomat::fn_traits(f).c_delete});
  return result;
}

inline int32_t CallbackWrapper::test_multiple_cb_args(std::function<int32_t()> f, std::function<int32_t(int32_t)> g) {
  auto result = diplomat::capi::CallbackWrapper_test_multiple_cb_args({new decltype(f)(f), diplomat::fn_traits(f).c_run_callback, diplomat::fn_traits(f).c_delete},
    {new decltype(g)(g), diplomat::fn_traits(g).c_run_callback, diplomat::fn_traits(g).c_delete});
  return result;
}

inline int32_t CallbackWrapper::test_str_cb_arg(std::function<int32_t(std::string_view)> f) {
  auto result = diplomat::capi::CallbackWrapper_test_str_cb_arg({new decltype(f)(f), diplomat::fn_traits(f).c_run_callback, diplomat::fn_traits(f).c_delete});
  return result;
}


inline diplomat::capi::CallbackWrapper CallbackWrapper::AsFFI() const {
  return diplomat::capi::CallbackWrapper {
    /* .cant_be_empty = */ cant_be_empty,
  };
}

inline CallbackWrapper CallbackWrapper::FromFFI(diplomat::capi::CallbackWrapper c_struct) {
  return CallbackWrapper {
    /* .cant_be_empty = */ c_struct.cant_be_empty,
  };
}


#endif // CallbackWrapper_HPP
