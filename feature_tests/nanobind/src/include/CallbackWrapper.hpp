#ifndef SOMELIB_CallbackWrapper_HPP
#define SOMELIB_CallbackWrapper_HPP

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
#include "FFIError.hpp"
#include "MyString.hpp"
#include "MyStruct.hpp"
#include "Opaque.hpp"
#include "PrimitiveStruct.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {
    typedef struct DiplomatCallback_CallbackWrapper_test_multi_arg_callback_fallible_f_result {union {int32_t ok; somelib::capi::FFIError err;}; bool is_ok;} DiplomatCallback_CallbackWrapper_test_multi_arg_callback_fallible_f_result;

    typedef struct DiplomatCallback_CallbackWrapper_test_multi_arg_callback_fallible_f {
        const void* data;
        DiplomatCallback_CallbackWrapper_test_multi_arg_callback_fallible_f_result (*run_callback)(const void*, int32_t );
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_multi_arg_callback_fallible_f;
    typedef struct DiplomatCallback_CallbackWrapper_test_no_args_fallible_h_result {union { somelib::capi::FFIError err;}; bool is_ok;} DiplomatCallback_CallbackWrapper_test_no_args_fallible_h_result;

    typedef struct DiplomatCallback_CallbackWrapper_test_no_args_fallible_h {
        const void* data;
        DiplomatCallback_CallbackWrapper_test_no_args_fallible_h_result (*run_callback)(const void*);
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_no_args_fallible_h;
    typedef struct DiplomatCallback_CallbackWrapper_test_cb_with_struct_fallible_f_result {union {int32_t ok; somelib::capi::FFIError err;}; bool is_ok;} DiplomatCallback_CallbackWrapper_test_cb_with_struct_fallible_f_result;

    typedef struct DiplomatCallback_CallbackWrapper_test_cb_with_struct_fallible_f {
        const void* data;
        DiplomatCallback_CallbackWrapper_test_cb_with_struct_fallible_f_result (*run_callback)(const void*, somelib::capi::CallbackTestingStruct );
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_cb_with_struct_fallible_f;
    typedef struct DiplomatCallback_CallbackWrapper_test_multiple_cb_args_fallible_f_result {union {int32_t ok; somelib::capi::FFIError err;}; bool is_ok;} DiplomatCallback_CallbackWrapper_test_multiple_cb_args_fallible_f_result;

    typedef struct DiplomatCallback_CallbackWrapper_test_multiple_cb_args_fallible_f {
        const void* data;
        DiplomatCallback_CallbackWrapper_test_multiple_cb_args_fallible_f_result (*run_callback)(const void*);
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_multiple_cb_args_fallible_f;
    typedef struct DiplomatCallback_CallbackWrapper_test_multiple_cb_args_fallible_g_result {union {int32_t ok; somelib::capi::FFIError err;}; bool is_ok;} DiplomatCallback_CallbackWrapper_test_multiple_cb_args_fallible_g_result;

    typedef struct DiplomatCallback_CallbackWrapper_test_multiple_cb_args_fallible_g {
        const void* data;
        DiplomatCallback_CallbackWrapper_test_multiple_cb_args_fallible_g_result (*run_callback)(const void*, int32_t );
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_multiple_cb_args_fallible_g;
    typedef struct DiplomatCallback_CallbackWrapper_test_str_cb_arg_fallible_f_result {union {int32_t ok; somelib::capi::FFIError err;}; bool is_ok;} DiplomatCallback_CallbackWrapper_test_str_cb_arg_fallible_f_result;

    typedef struct DiplomatCallback_CallbackWrapper_test_str_cb_arg_fallible_f {
        const void* data;
        DiplomatCallback_CallbackWrapper_test_str_cb_arg_fallible_f_result (*run_callback)(const void*, somelib::diplomat::capi::DiplomatStringView );
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_str_cb_arg_fallible_f;
    typedef struct DiplomatCallback_CallbackWrapper_test_opaque_cb_arg_fallible_cb_result {union { somelib::capi::FFIError err;}; bool is_ok;} DiplomatCallback_CallbackWrapper_test_opaque_cb_arg_fallible_cb_result;

    typedef struct DiplomatCallback_CallbackWrapper_test_opaque_cb_arg_fallible_cb {
        const void* data;
        DiplomatCallback_CallbackWrapper_test_opaque_cb_arg_fallible_cb_result (*run_callback)(const void*, somelib::capi::MyString* );
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_opaque_cb_arg_fallible_cb;
    typedef struct DiplomatCallback_CallbackWrapper_test_result_opaque_t_result {union {const somelib::capi::Opaque* ok; somelib::capi::FFIError err;}; bool is_ok;} DiplomatCallback_CallbackWrapper_test_result_opaque_t_result;

    typedef struct DiplomatCallback_CallbackWrapper_test_result_opaque_t {
        const void* data;
        DiplomatCallback_CallbackWrapper_test_result_opaque_t_result (*run_callback)(const void*);
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_result_opaque_t;
    typedef struct DiplomatCallback_CallbackWrapper_test_str_conversion_t_result {union {somelib::diplomat::capi::DiplomatStringView ok; somelib::capi::FFIError err;}; bool is_ok;} DiplomatCallback_CallbackWrapper_test_str_conversion_t_result;

    typedef struct DiplomatCallback_CallbackWrapper_test_str_conversion_t {
        const void* data;
        DiplomatCallback_CallbackWrapper_test_str_conversion_t_result (*run_callback)(const void*);
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_str_conversion_t;
    typedef struct DiplomatCallback_CallbackWrapper_test_slice_conversion_t_result {union {somelib::diplomat::capi::DiplomatF64View ok; somelib::capi::FFIError err;}; bool is_ok;} DiplomatCallback_CallbackWrapper_test_slice_conversion_t_result;

    typedef struct DiplomatCallback_CallbackWrapper_test_slice_conversion_t {
        const void* data;
        DiplomatCallback_CallbackWrapper_test_slice_conversion_t_result (*run_callback)(const void*);
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_slice_conversion_t;
    typedef struct DiplomatCallback_CallbackWrapper_test_result_option_struct_conversion_t_result {union {somelib::capi::MyStruct_option ok; somelib::capi::FFIError err;}; bool is_ok;} DiplomatCallback_CallbackWrapper_test_result_option_struct_conversion_t_result;

    typedef struct DiplomatCallback_CallbackWrapper_test_result_option_struct_conversion_t {
        const void* data;
        DiplomatCallback_CallbackWrapper_test_result_option_struct_conversion_t_result (*run_callback)(const void*);
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_result_option_struct_conversion_t;
    typedef struct DiplomatCallback_CallbackWrapper_test_struct_slice_conversion_t_result {union {somelib::capi::DiplomatPrimitiveStructView ok; somelib::capi::FFIError err;}; bool is_ok;} DiplomatCallback_CallbackWrapper_test_struct_slice_conversion_t_result;

    typedef struct DiplomatCallback_CallbackWrapper_test_struct_slice_conversion_t {
        const void* data;
        DiplomatCallback_CallbackWrapper_test_struct_slice_conversion_t_result (*run_callback)(const void*);
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_struct_slice_conversion_t;
    typedef struct DiplomatCallback_CallbackWrapper_test_ffi_error_t_result {union { somelib::capi::FFIError err;}; bool is_ok;} DiplomatCallback_CallbackWrapper_test_ffi_error_t_result;

    typedef struct DiplomatCallback_CallbackWrapper_test_ffi_error_t {
        const void* data;
        DiplomatCallback_CallbackWrapper_test_ffi_error_t_result (*run_callback)(const void*);
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_ffi_error_t;
    typedef struct DiplomatCallback_CallbackWrapper_test_ffi_error_as_ok_t_result {union { somelib::capi::FFIError err;}; bool is_ok;} DiplomatCallback_CallbackWrapper_test_ffi_error_as_ok_t_result;

    typedef struct DiplomatCallback_CallbackWrapper_test_ffi_error_as_ok_t {
        const void* data;
        DiplomatCallback_CallbackWrapper_test_ffi_error_as_ok_t_result (*run_callback)(const void*);
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_ffi_error_as_ok_t;

    int32_t CallbackWrapper_test_multi_arg_callback_fallible(DiplomatCallback_CallbackWrapper_test_multi_arg_callback_fallible_f f_cb_wrap, int32_t x);

    int32_t CallbackWrapper_test_no_args_fallible(DiplomatCallback_CallbackWrapper_test_no_args_fallible_h h_cb_wrap);

    int32_t CallbackWrapper_test_cb_with_struct_fallible(DiplomatCallback_CallbackWrapper_test_cb_with_struct_fallible_f f_cb_wrap);

    int32_t CallbackWrapper_test_multiple_cb_args_fallible(DiplomatCallback_CallbackWrapper_test_multiple_cb_args_fallible_f f_cb_wrap, DiplomatCallback_CallbackWrapper_test_multiple_cb_args_fallible_g g_cb_wrap);

    int32_t CallbackWrapper_test_str_cb_arg_fallible(DiplomatCallback_CallbackWrapper_test_str_cb_arg_fallible_f f_cb_wrap);

    void CallbackWrapper_test_opaque_cb_arg_fallible(DiplomatCallback_CallbackWrapper_test_opaque_cb_arg_fallible_cb cb_cb_wrap, somelib::capi::MyString* a);

    void CallbackWrapper_test_result_opaque(DiplomatCallback_CallbackWrapper_test_result_opaque_t t_cb_wrap, somelib::diplomat::capi::DiplomatWrite* write);

    void CallbackWrapper_test_str_conversion(DiplomatCallback_CallbackWrapper_test_str_conversion_t t_cb_wrap);

    void CallbackWrapper_test_slice_conversion(DiplomatCallback_CallbackWrapper_test_slice_conversion_t t_cb_wrap);

    void CallbackWrapper_test_result_option_struct_conversion(DiplomatCallback_CallbackWrapper_test_result_option_struct_conversion_t t_cb_wrap);

    void CallbackWrapper_test_struct_slice_conversion(DiplomatCallback_CallbackWrapper_test_struct_slice_conversion_t t_cb_wrap);

    typedef struct CallbackWrapper_test_ffi_error_result {union { somelib::capi::FFIError err;}; bool is_ok;} CallbackWrapper_test_ffi_error_result;
    CallbackWrapper_test_ffi_error_result CallbackWrapper_test_ffi_error(DiplomatCallback_CallbackWrapper_test_ffi_error_t t_cb_wrap);

    somelib::capi::FFIError CallbackWrapper_test_ffi_error_as_ok(DiplomatCallback_CallbackWrapper_test_ffi_error_as_ok_t t_cb_wrap);

    } // extern "C"
} // namespace capi
} // namespace

inline int32_t somelib::CallbackWrapper::test_multi_arg_callback_fallible(std::function<somelib::diplomat::result<int32_t, somelib::FFIError>(int32_t)> f, int32_t x) {
    auto result = somelib::capi::CallbackWrapper_test_multi_arg_callback_fallible({new decltype(f)(std::move(f)), somelib::diplomat::fn_traits(f).template c_run_callback_result<int32_t, somelib::FFIError, somelib::capi::DiplomatCallback_CallbackWrapper_test_multi_arg_callback_fallible_f_result>, somelib::diplomat::fn_traits(f).c_delete},
        x);
    return result;
}

inline int32_t somelib::CallbackWrapper::test_no_args_fallible(std::function<somelib::diplomat::result<std::monostate, somelib::FFIError>()> h) {
    auto result = somelib::capi::CallbackWrapper_test_no_args_fallible({new decltype(h)(std::move(h)), somelib::diplomat::fn_traits(h).template c_run_callback_result<std::monostate, somelib::FFIError, somelib::capi::DiplomatCallback_CallbackWrapper_test_no_args_fallible_h_result>, somelib::diplomat::fn_traits(h).c_delete});
    return result;
}

inline int32_t somelib::CallbackWrapper::test_cb_with_struct_fallible(std::function<somelib::diplomat::result<int32_t, somelib::FFIError>(somelib::CallbackTestingStruct)> f) {
    auto result = somelib::capi::CallbackWrapper_test_cb_with_struct_fallible({new decltype(f)(std::move(f)), somelib::diplomat::fn_traits(f).template c_run_callback_result<int32_t, somelib::FFIError, somelib::capi::DiplomatCallback_CallbackWrapper_test_cb_with_struct_fallible_f_result>, somelib::diplomat::fn_traits(f).c_delete});
    return result;
}

inline int32_t somelib::CallbackWrapper::test_multiple_cb_args_fallible(std::function<somelib::diplomat::result<int32_t, somelib::FFIError>()> f, std::function<somelib::diplomat::result<int32_t, somelib::FFIError>(int32_t)> g) {
    auto result = somelib::capi::CallbackWrapper_test_multiple_cb_args_fallible({new decltype(f)(std::move(f)), somelib::diplomat::fn_traits(f).template c_run_callback_result<int32_t, somelib::FFIError, somelib::capi::DiplomatCallback_CallbackWrapper_test_multiple_cb_args_fallible_f_result>, somelib::diplomat::fn_traits(f).c_delete},
        {new decltype(g)(std::move(g)), somelib::diplomat::fn_traits(g).template c_run_callback_result<int32_t, somelib::FFIError, somelib::capi::DiplomatCallback_CallbackWrapper_test_multiple_cb_args_fallible_g_result>, somelib::diplomat::fn_traits(g).c_delete});
    return result;
}

inline int32_t somelib::CallbackWrapper::test_str_cb_arg_fallible(std::function<somelib::diplomat::result<int32_t, somelib::FFIError>(std::string_view)> f) {
    auto result = somelib::capi::CallbackWrapper_test_str_cb_arg_fallible({new decltype(f)(std::move(f)), somelib::diplomat::fn_traits(f).template c_run_callback_result<int32_t, somelib::FFIError, somelib::capi::DiplomatCallback_CallbackWrapper_test_str_cb_arg_fallible_f_result>, somelib::diplomat::fn_traits(f).c_delete});
    return result;
}

inline void somelib::CallbackWrapper::test_opaque_cb_arg_fallible(std::function<somelib::diplomat::result<std::monostate, somelib::FFIError>(somelib::MyString&)> cb, somelib::MyString& a) {
    somelib::capi::CallbackWrapper_test_opaque_cb_arg_fallible({new decltype(cb)(std::move(cb)), somelib::diplomat::fn_traits(cb).template c_run_callback_result<std::monostate, somelib::FFIError, somelib::capi::DiplomatCallback_CallbackWrapper_test_opaque_cb_arg_fallible_cb_result>, somelib::diplomat::fn_traits(cb).c_delete},
        a.AsFFI());
}

inline std::string somelib::CallbackWrapper::test_result_opaque(std::function<somelib::diplomat::result<const somelib::Opaque&, somelib::FFIError>()> t) {
    std::string output;
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteFromString(output);
    somelib::capi::CallbackWrapper_test_result_opaque({new decltype(t)(std::move(t)), somelib::diplomat::fn_traits(t).template c_run_callback_result<const somelib::Opaque&, somelib::FFIError, somelib::capi::DiplomatCallback_CallbackWrapper_test_result_opaque_t_result>, somelib::diplomat::fn_traits(t).c_delete},
        &write);
    return output;
}
template<typename W>
inline void somelib::CallbackWrapper::test_result_opaque_write(std::function<somelib::diplomat::result<const somelib::Opaque&, somelib::FFIError>()> t, W& writeable) {
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteTrait<W>::Construct(writeable);
    somelib::capi::CallbackWrapper_test_result_opaque({new decltype(t)(std::move(t)), somelib::diplomat::fn_traits(t).template c_run_callback_result<const somelib::Opaque&, somelib::FFIError, somelib::capi::DiplomatCallback_CallbackWrapper_test_result_opaque_t_result>, somelib::diplomat::fn_traits(t).c_delete},
        &write);
}

inline void somelib::CallbackWrapper::test_str_conversion(std::function<somelib::diplomat::result<std::string_view, somelib::FFIError>()> t) {
    somelib::capi::CallbackWrapper_test_str_conversion({new decltype(t)(std::move(t)), somelib::diplomat::fn_traits(t).template c_run_callback_result<std::string_view, somelib::FFIError, somelib::capi::DiplomatCallback_CallbackWrapper_test_str_conversion_t_result>, somelib::diplomat::fn_traits(t).c_delete});
}

inline void somelib::CallbackWrapper::test_slice_conversion(std::function<somelib::diplomat::result<somelib::diplomat::span<const double>, somelib::FFIError>()> t) {
    somelib::capi::CallbackWrapper_test_slice_conversion({new decltype(t)(std::move(t)), somelib::diplomat::fn_traits(t).template c_run_callback_result<somelib::diplomat::span<const double>, somelib::FFIError, somelib::capi::DiplomatCallback_CallbackWrapper_test_slice_conversion_t_result>, somelib::diplomat::fn_traits(t).c_delete});
}

inline void somelib::CallbackWrapper::test_result_option_struct_conversion(std::function<somelib::diplomat::result<std::optional<somelib::MyStruct>, somelib::FFIError>()> t) {
    somelib::capi::CallbackWrapper_test_result_option_struct_conversion({new decltype(t)(std::move(t)), somelib::diplomat::fn_traits(t).template c_run_callback_result<std::optional<somelib::MyStruct>, somelib::FFIError, somelib::capi::DiplomatCallback_CallbackWrapper_test_result_option_struct_conversion_t_result>, somelib::diplomat::fn_traits(t).c_delete});
}

inline void somelib::CallbackWrapper::test_struct_slice_conversion(std::function<somelib::diplomat::result<somelib::diplomat::span<const somelib::PrimitiveStruct>, somelib::FFIError>()> t) {
    somelib::capi::CallbackWrapper_test_struct_slice_conversion({new decltype(t)(std::move(t)), somelib::diplomat::fn_traits(t).template c_run_callback_result<somelib::diplomat::span<const somelib::PrimitiveStruct>, somelib::FFIError, somelib::capi::DiplomatCallback_CallbackWrapper_test_struct_slice_conversion_t_result>, somelib::diplomat::fn_traits(t).c_delete});
}

inline somelib::diplomat::result<std::monostate, somelib::FFIError> somelib::CallbackWrapper::test_ffi_error(std::function<somelib::diplomat::result<std::monostate, somelib::FFIError>()> t) {
    auto result = somelib::capi::CallbackWrapper_test_ffi_error({new decltype(t)(std::move(t)), somelib::diplomat::fn_traits(t).template c_run_callback_result<std::monostate, somelib::FFIError, somelib::capi::DiplomatCallback_CallbackWrapper_test_ffi_error_t_result>, somelib::diplomat::fn_traits(t).c_delete});
    return result.is_ok ? somelib::diplomat::result<std::monostate, somelib::FFIError>(somelib::diplomat::Ok<std::monostate>()) : somelib::diplomat::result<std::monostate, somelib::FFIError>(somelib::diplomat::Err<somelib::FFIError>(somelib::FFIError::FromFFI(result.err)));
}

inline somelib::FFIError somelib::CallbackWrapper::test_ffi_error_as_ok(std::function<somelib::diplomat::result<std::monostate, somelib::FFIError>()> t) {
    auto result = somelib::capi::CallbackWrapper_test_ffi_error_as_ok({new decltype(t)(std::move(t)), somelib::diplomat::fn_traits(t).template c_run_callback_result<std::monostate, somelib::FFIError, somelib::capi::DiplomatCallback_CallbackWrapper_test_ffi_error_as_ok_t_result>, somelib::diplomat::fn_traits(t).c_delete});
    return somelib::FFIError::FromFFI(result);
}


inline somelib::capi::CallbackWrapper somelib::CallbackWrapper::AsFFI() const {
    return somelib::capi::CallbackWrapper {
        /* .cant_be_empty = */ cant_be_empty,
    };
}

inline somelib::CallbackWrapper somelib::CallbackWrapper::FromFFI(somelib::capi::CallbackWrapper c_struct) {
    return somelib::CallbackWrapper {
        /* .cant_be_empty = */ c_struct.cant_be_empty,
    };
}


#endif // SOMELIB_CallbackWrapper_HPP
