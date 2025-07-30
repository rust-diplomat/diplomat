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
#include <cstdlib>
#include "CallbackTestingStruct.hpp"
#include "MyString.hpp"
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
    typedef struct DiplomatCallback_CallbackWrapper_test_opaque_cb_arg_cb {
        const void* data;
        void (*run_callback)(const void*, diplomat::capi::MyString* );
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_opaque_cb_arg_cb;
    typedef struct DiplomatCallback_CallbackWrapper_test_slice_cb_arg_f {
        const void* data;
        void (*run_callback)(const void*, diplomat::capi::DiplomatU8View );
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_slice_cb_arg_f;
    typedef struct DiplomatCallback_CallbackWrapper_test_result_output_t {
        const void* data;
        typedef struct DiplomatCallback_CallbackWrapper_test_result_output_t_result_void_result { bool is_ok;} DiplomatCallback_CallbackWrapper_test_result_output_t_result_void_result;
    DiplomatCallback_CallbackWrapper_test_result_output_t_result_void_result (*run_callback)(const void*);
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_result_output_t;

    int32_t CallbackWrapper_test_multi_arg_callback(DiplomatCallback_CallbackWrapper_test_multi_arg_callback_f f_cb_wrap, int32_t x);

    int32_t CallbackWrapper_test_no_args(DiplomatCallback_CallbackWrapper_test_no_args_h h_cb_wrap);

    int32_t CallbackWrapper_test_cb_with_struct(DiplomatCallback_CallbackWrapper_test_cb_with_struct_f f_cb_wrap);

    int32_t CallbackWrapper_test_multiple_cb_args(DiplomatCallback_CallbackWrapper_test_multiple_cb_args_f f_cb_wrap, DiplomatCallback_CallbackWrapper_test_multiple_cb_args_g g_cb_wrap);

    int32_t CallbackWrapper_test_str_cb_arg(DiplomatCallback_CallbackWrapper_test_str_cb_arg_f f_cb_wrap);

    void CallbackWrapper_test_opaque_cb_arg(DiplomatCallback_CallbackWrapper_test_opaque_cb_arg_cb cb_cb_wrap, diplomat::capi::MyString* a);

    void CallbackWrapper_test_slice_cb_arg(diplomat::capi::DiplomatU8View arg, DiplomatCallback_CallbackWrapper_test_slice_cb_arg_f f_cb_wrap);

    void CallbackWrapper_test_result_output(DiplomatCallback_CallbackWrapper_test_result_output_t t_cb_wrap);

    } // extern "C"
} // namespace capi
} // namespace

inline int32_t CallbackWrapper::test_multi_arg_callback(std::function<int32_t(int32_t)> f, int32_t x) {
  auto result = diplomat::capi::CallbackWrapper_test_multi_arg_callback({new decltype(f)(std::move(f)), diplomat::fn_traits(f).c_run_callback, diplomat::fn_traits(f).c_delete},
    x);
  return result;
}

inline int32_t CallbackWrapper::test_no_args(std::function<void()> h) {
  auto result = diplomat::capi::CallbackWrapper_test_no_args({new decltype(h)(std::move(h)), diplomat::fn_traits(h).c_run_callback, diplomat::fn_traits(h).c_delete});
  return result;
}

inline int32_t CallbackWrapper::test_cb_with_struct(std::function<int32_t(CallbackTestingStruct)> f) {
  auto result = diplomat::capi::CallbackWrapper_test_cb_with_struct({new decltype(f)(std::move(f)), diplomat::fn_traits(f).c_run_callback, diplomat::fn_traits(f).c_delete});
  return result;
}

inline int32_t CallbackWrapper::test_multiple_cb_args(std::function<int32_t()> f, std::function<int32_t(int32_t)> g) {
  auto result = diplomat::capi::CallbackWrapper_test_multiple_cb_args({new decltype(f)(std::move(f)), diplomat::fn_traits(f).c_run_callback, diplomat::fn_traits(f).c_delete},
    {new decltype(g)(std::move(g)), diplomat::fn_traits(g).c_run_callback, diplomat::fn_traits(g).c_delete});
  return result;
}

inline int32_t CallbackWrapper::test_str_cb_arg(std::function<int32_t(std::string_view)> f) {
  auto result = diplomat::capi::CallbackWrapper_test_str_cb_arg({new decltype(f)(std::move(f)), diplomat::fn_traits(f).c_run_callback, diplomat::fn_traits(f).c_delete});
  return result;
}

inline void CallbackWrapper::test_opaque_cb_arg(std::function<void(MyString&)> cb, MyString& a) {
  diplomat::capi::CallbackWrapper_test_opaque_cb_arg({new decltype(cb)(std::move(cb)), diplomat::fn_traits(cb).c_run_callback, diplomat::fn_traits(cb).c_delete},
    a.AsFFI());
}

inline void CallbackWrapper::test_slice_cb_arg(diplomat::span<const uint8_t> arg, std::function<void(diplomat::span<const uint8_t>)> f) {
  diplomat::capi::CallbackWrapper_test_slice_cb_arg({arg.data(), arg.size()},
    {new decltype(f)(std::move(f)), diplomat::fn_traits(f).c_run_callback, diplomat::fn_traits(f).c_delete});
}

inline void CallbackWrapper::test_result_output(std::function<diplomat::result<std::monostate, std::monostate>()> t) {
  diplomat::capi::CallbackWrapper_test_result_output({new decltype(t)(std::move(t)), diplomat::fn_traits(t).c_run_callback, diplomat::fn_traits(t).c_delete});
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
