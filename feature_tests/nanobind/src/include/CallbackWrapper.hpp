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
#include "MyString.hpp"
#include "MyStructContainingAnOption.hpp"
#include "Opaque.hpp"
#include "PrimitiveStruct.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
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
        int32_t (*run_callback)(const void*, somelib::capi::CallbackTestingStruct );
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
        int32_t (*run_callback)(const void*, somelib::diplomat::capi::DiplomatStringView );
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_str_cb_arg_f;
    typedef struct DiplomatCallback_CallbackWrapper_test_opaque_cb_arg_cb {
        const void* data;
        void (*run_callback)(const void*, somelib::capi::MyString* );
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_opaque_cb_arg_cb;
    typedef struct DiplomatCallback_CallbackWrapper_test_slice_cb_arg_f {
        const void* data;
        void (*run_callback)(const void*, somelib::diplomat::capi::DiplomatU8View );
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_slice_cb_arg_f;
    typedef struct DiplomatCallback_CallbackWrapper_test_result_output_t_result { bool is_ok;} DiplomatCallback_CallbackWrapper_test_result_output_t_result;

    typedef struct DiplomatCallback_CallbackWrapper_test_result_output_t {
        const void* data;
        DiplomatCallback_CallbackWrapper_test_result_output_t_result (*run_callback)(const void*);
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_result_output_t;
    typedef struct DiplomatCallback_CallbackWrapper_test_result_usize_output_t_result {union {size_t ok; }; bool is_ok;} DiplomatCallback_CallbackWrapper_test_result_usize_output_t_result;

    typedef struct DiplomatCallback_CallbackWrapper_test_result_usize_output_t {
        const void* data;
        DiplomatCallback_CallbackWrapper_test_result_usize_output_t_result (*run_callback)(const void*);
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_result_usize_output_t;
    typedef struct DiplomatCallback_CallbackWrapper_test_option_output_t_result { bool is_ok;} DiplomatCallback_CallbackWrapper_test_option_output_t_result;

    typedef struct DiplomatCallback_CallbackWrapper_test_option_output_t {
        const void* data;
        DiplomatCallback_CallbackWrapper_test_option_output_t_result (*run_callback)(const void*);
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_option_output_t;
    typedef struct DiplomatCallback_CallbackWrapper_test_diplomat_option_output_t_result {union {uint32_t ok; }; bool is_ok;} DiplomatCallback_CallbackWrapper_test_diplomat_option_output_t_result;

    typedef struct DiplomatCallback_CallbackWrapper_test_diplomat_option_output_t {
        const void* data;
        DiplomatCallback_CallbackWrapper_test_diplomat_option_output_t_result (*run_callback)(const void*);
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_diplomat_option_output_t;
    typedef struct DiplomatCallback_CallbackWrapper_test_option_opaque_t {
        const void* data;
        const somelib::capi::Opaque* (*run_callback)(const void*);
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_option_opaque_t;
    typedef struct DiplomatCallback_CallbackWrapper_test_diplomat_result_t_result {union {size_t ok; size_t err;}; bool is_ok;} DiplomatCallback_CallbackWrapper_test_diplomat_result_t_result;

    typedef struct DiplomatCallback_CallbackWrapper_test_diplomat_result_t {
        const void* data;
        DiplomatCallback_CallbackWrapper_test_diplomat_result_t_result (*run_callback)(const void*);
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_diplomat_result_t;
    typedef struct DiplomatCallback_CallbackWrapper_test_result_opaque_t_result {union {const somelib::capi::Opaque* ok; }; bool is_ok;} DiplomatCallback_CallbackWrapper_test_result_opaque_t_result;

    typedef struct DiplomatCallback_CallbackWrapper_test_result_opaque_t {
        const void* data;
        DiplomatCallback_CallbackWrapper_test_result_opaque_t_result (*run_callback)(const void*);
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_result_opaque_t;
    typedef struct DiplomatCallback_CallbackWrapper_test_inner_conversion_t_result {union {somelib::capi::MyStructContainingAnOption ok; size_t err;}; bool is_ok;} DiplomatCallback_CallbackWrapper_test_inner_conversion_t_result;

    typedef struct DiplomatCallback_CallbackWrapper_test_inner_conversion_t {
        const void* data;
        DiplomatCallback_CallbackWrapper_test_inner_conversion_t_result (*run_callback)(const void*);
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_inner_conversion_t;
    typedef struct DiplomatCallback_CallbackWrapper_test_str_conversion_t_result {union {somelib::diplomat::capi::DiplomatStringView ok; }; bool is_ok;} DiplomatCallback_CallbackWrapper_test_str_conversion_t_result;

    typedef struct DiplomatCallback_CallbackWrapper_test_str_conversion_t {
        const void* data;
        DiplomatCallback_CallbackWrapper_test_str_conversion_t_result (*run_callback)(const void*);
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_str_conversion_t;
    typedef struct DiplomatCallback_CallbackWrapper_test_slice_conversion_t_result {union {somelib::diplomat::capi::DiplomatF64View ok; }; bool is_ok;} DiplomatCallback_CallbackWrapper_test_slice_conversion_t_result;

    typedef struct DiplomatCallback_CallbackWrapper_test_slice_conversion_t {
        const void* data;
        DiplomatCallback_CallbackWrapper_test_slice_conversion_t_result (*run_callback)(const void*);
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_slice_conversion_t;
    typedef struct DiplomatCallback_CallbackWrapper_test_struct_slice_conversion_t_result {union {somelib::capi::DiplomatPrimitiveStructView ok; }; bool is_ok;} DiplomatCallback_CallbackWrapper_test_struct_slice_conversion_t_result;

    typedef struct DiplomatCallback_CallbackWrapper_test_struct_slice_conversion_t {
        const void* data;
        DiplomatCallback_CallbackWrapper_test_struct_slice_conversion_t_result (*run_callback)(const void*);
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_struct_slice_conversion_t;
    typedef struct DiplomatCallback_CallbackWrapper_test_opaque_result_error_t_result {union { const somelib::capi::Opaque* err;}; bool is_ok;} DiplomatCallback_CallbackWrapper_test_opaque_result_error_t_result;

    typedef struct DiplomatCallback_CallbackWrapper_test_opaque_result_error_t {
        const void* data;
        DiplomatCallback_CallbackWrapper_test_opaque_result_error_t_result (*run_callback)(const void*);
        void (*destructor)(const void*);
    } DiplomatCallback_CallbackWrapper_test_opaque_result_error_t;

    int32_t CallbackWrapper_test_multi_arg_callback(DiplomatCallback_CallbackWrapper_test_multi_arg_callback_f f_cb_wrap, int32_t x);

    int32_t CallbackWrapper_test_no_args(DiplomatCallback_CallbackWrapper_test_no_args_h h_cb_wrap);

    int32_t CallbackWrapper_test_cb_with_struct(DiplomatCallback_CallbackWrapper_test_cb_with_struct_f f_cb_wrap);

    int32_t CallbackWrapper_test_multiple_cb_args(DiplomatCallback_CallbackWrapper_test_multiple_cb_args_f f_cb_wrap, DiplomatCallback_CallbackWrapper_test_multiple_cb_args_g g_cb_wrap);

    int32_t CallbackWrapper_test_str_cb_arg(DiplomatCallback_CallbackWrapper_test_str_cb_arg_f f_cb_wrap);

    void CallbackWrapper_test_opaque_cb_arg(DiplomatCallback_CallbackWrapper_test_opaque_cb_arg_cb cb_cb_wrap, somelib::capi::MyString* a);

    void CallbackWrapper_test_slice_cb_arg(somelib::diplomat::capi::DiplomatU8View arg, DiplomatCallback_CallbackWrapper_test_slice_cb_arg_f f_cb_wrap);

    void CallbackWrapper_test_result_output(DiplomatCallback_CallbackWrapper_test_result_output_t t_cb_wrap);

    void CallbackWrapper_test_result_usize_output(DiplomatCallback_CallbackWrapper_test_result_usize_output_t t_cb_wrap);

    void CallbackWrapper_test_option_output(DiplomatCallback_CallbackWrapper_test_option_output_t t_cb_wrap);

    void CallbackWrapper_test_diplomat_option_output(DiplomatCallback_CallbackWrapper_test_diplomat_option_output_t t_cb_wrap);

    void CallbackWrapper_test_option_opaque(DiplomatCallback_CallbackWrapper_test_option_opaque_t t_cb_wrap, somelib::diplomat::capi::DiplomatWrite* write);

    void CallbackWrapper_test_diplomat_result(DiplomatCallback_CallbackWrapper_test_diplomat_result_t t_cb_wrap);

    void CallbackWrapper_test_result_opaque(DiplomatCallback_CallbackWrapper_test_result_opaque_t t_cb_wrap, somelib::diplomat::capi::DiplomatWrite* write);

    void CallbackWrapper_test_inner_conversion(DiplomatCallback_CallbackWrapper_test_inner_conversion_t t_cb_wrap);

    void CallbackWrapper_test_str_conversion(DiplomatCallback_CallbackWrapper_test_str_conversion_t t_cb_wrap);

    void CallbackWrapper_test_slice_conversion(DiplomatCallback_CallbackWrapper_test_slice_conversion_t t_cb_wrap);

    void CallbackWrapper_test_struct_slice_conversion(DiplomatCallback_CallbackWrapper_test_struct_slice_conversion_t t_cb_wrap);

    void CallbackWrapper_test_opaque_result_error(DiplomatCallback_CallbackWrapper_test_opaque_result_error_t t_cb_wrap, somelib::diplomat::capi::DiplomatWrite* write);

    } // extern "C"
} // namespace capi
} // namespace

inline int32_t somelib::CallbackWrapper::test_multi_arg_callback(std::function<int32_t(int32_t)> f, int32_t x) {
    auto result = somelib::capi::CallbackWrapper_test_multi_arg_callback({new decltype(f)(std::move(f)), somelib::diplomat::fn_traits(f).c_run_callback, somelib::diplomat::fn_traits(f).c_delete},
        x);
    return result;
}

inline int32_t somelib::CallbackWrapper::test_no_args(std::function<void()> h) {
    auto result = somelib::capi::CallbackWrapper_test_no_args({new decltype(h)(std::move(h)), somelib::diplomat::fn_traits(h).c_run_callback, somelib::diplomat::fn_traits(h).c_delete});
    return result;
}

inline int32_t somelib::CallbackWrapper::test_cb_with_struct(std::function<int32_t(somelib::CallbackTestingStruct)> f) {
    auto result = somelib::capi::CallbackWrapper_test_cb_with_struct({new decltype(f)(std::move(f)), somelib::diplomat::fn_traits(f).c_run_callback, somelib::diplomat::fn_traits(f).c_delete});
    return result;
}

inline int32_t somelib::CallbackWrapper::test_multiple_cb_args(std::function<int32_t()> f, std::function<int32_t(int32_t)> g) {
    auto result = somelib::capi::CallbackWrapper_test_multiple_cb_args({new decltype(f)(std::move(f)), somelib::diplomat::fn_traits(f).c_run_callback, somelib::diplomat::fn_traits(f).c_delete},
        {new decltype(g)(std::move(g)), somelib::diplomat::fn_traits(g).c_run_callback, somelib::diplomat::fn_traits(g).c_delete});
    return result;
}

inline int32_t somelib::CallbackWrapper::test_str_cb_arg(std::function<int32_t(std::string_view)> f) {
    auto result = somelib::capi::CallbackWrapper_test_str_cb_arg({new decltype(f)(std::move(f)), somelib::diplomat::fn_traits(f).c_run_callback, somelib::diplomat::fn_traits(f).c_delete});
    return result;
}

inline void somelib::CallbackWrapper::test_opaque_cb_arg(std::function<void(somelib::MyString&)> cb, somelib::MyString& a) {
    somelib::capi::CallbackWrapper_test_opaque_cb_arg({new decltype(cb)(std::move(cb)), somelib::diplomat::fn_traits(cb).c_run_callback, somelib::diplomat::fn_traits(cb).c_delete},
        a.AsFFI());
}

inline void somelib::CallbackWrapper::test_slice_cb_arg(somelib::diplomat::span<const uint8_t> arg, std::function<void(somelib::diplomat::span<const uint8_t>)> f) {
    somelib::capi::CallbackWrapper_test_slice_cb_arg({arg.data(), arg.size()},
        {new decltype(f)(std::move(f)), somelib::diplomat::fn_traits(f).c_run_callback, somelib::diplomat::fn_traits(f).c_delete});
}

inline void somelib::CallbackWrapper::test_result_output(std::function<somelib::diplomat::result<std::monostate, std::monostate>()> t) {
    somelib::capi::CallbackWrapper_test_result_output({new decltype(t)(std::move(t)), somelib::diplomat::fn_traits(t).template c_run_callback_result<std::monostate, std::monostate, somelib::capi::DiplomatCallback_CallbackWrapper_test_result_output_t_result>, somelib::diplomat::fn_traits(t).c_delete});
}

inline void somelib::CallbackWrapper::test_result_usize_output(std::function<somelib::diplomat::result<size_t, std::monostate>()> t) {
    somelib::capi::CallbackWrapper_test_result_usize_output({new decltype(t)(std::move(t)), somelib::diplomat::fn_traits(t).template c_run_callback_result<size_t, std::monostate, somelib::capi::DiplomatCallback_CallbackWrapper_test_result_usize_output_t_result>, somelib::diplomat::fn_traits(t).c_delete});
}

inline void somelib::CallbackWrapper::test_option_output(std::function<std::optional<std::monostate>()> t) {
    somelib::capi::CallbackWrapper_test_option_output({new decltype(t)(std::move(t)), somelib::diplomat::fn_traits(t).template c_run_callback_diplomat_option<std::monostate, somelib::capi::DiplomatCallback_CallbackWrapper_test_option_output_t_result>, somelib::diplomat::fn_traits(t).c_delete});
}

inline void somelib::CallbackWrapper::test_diplomat_option_output(std::function<std::optional<uint32_t>()> t) {
    somelib::capi::CallbackWrapper_test_diplomat_option_output({new decltype(t)(std::move(t)), somelib::diplomat::fn_traits(t).template c_run_callback_diplomat_option<uint32_t, somelib::capi::DiplomatCallback_CallbackWrapper_test_diplomat_option_output_t_result>, somelib::diplomat::fn_traits(t).c_delete});
}

inline std::string somelib::CallbackWrapper::test_option_opaque(std::function<const somelib::Opaque*()> t) {
    std::string output;
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteFromString(output);
    somelib::capi::CallbackWrapper_test_option_opaque({new decltype(t)(std::move(t)), somelib::diplomat::fn_traits(t).template c_run_callback_diplomat_opaque<const somelib::capi::Opaque*>, somelib::diplomat::fn_traits(t).c_delete},
        &write);
    return output;
}
template<typename W>
inline void somelib::CallbackWrapper::test_option_opaque_write(std::function<const somelib::Opaque*()> t, W& writeable) {
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteTrait<W>::Construct(writeable);
    somelib::capi::CallbackWrapper_test_option_opaque({new decltype(t)(std::move(t)), somelib::diplomat::fn_traits(t).template c_run_callback_diplomat_opaque<const somelib::capi::Opaque*>, somelib::diplomat::fn_traits(t).c_delete},
        &write);
}

inline void somelib::CallbackWrapper::test_diplomat_result(std::function<somelib::diplomat::result<size_t, size_t>()> t) {
    somelib::capi::CallbackWrapper_test_diplomat_result({new decltype(t)(std::move(t)), somelib::diplomat::fn_traits(t).template c_run_callback_result<size_t, size_t, somelib::capi::DiplomatCallback_CallbackWrapper_test_diplomat_result_t_result>, somelib::diplomat::fn_traits(t).c_delete});
}

inline std::string somelib::CallbackWrapper::test_result_opaque(std::function<somelib::diplomat::result<const somelib::Opaque&, std::monostate>()> t) {
    std::string output;
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteFromString(output);
    somelib::capi::CallbackWrapper_test_result_opaque({new decltype(t)(std::move(t)), somelib::diplomat::fn_traits(t).template c_run_callback_result<const somelib::Opaque&, std::monostate, somelib::capi::DiplomatCallback_CallbackWrapper_test_result_opaque_t_result>, somelib::diplomat::fn_traits(t).c_delete},
        &write);
    return output;
}
template<typename W>
inline void somelib::CallbackWrapper::test_result_opaque_write(std::function<somelib::diplomat::result<const somelib::Opaque&, std::monostate>()> t, W& writeable) {
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteTrait<W>::Construct(writeable);
    somelib::capi::CallbackWrapper_test_result_opaque({new decltype(t)(std::move(t)), somelib::diplomat::fn_traits(t).template c_run_callback_result<const somelib::Opaque&, std::monostate, somelib::capi::DiplomatCallback_CallbackWrapper_test_result_opaque_t_result>, somelib::diplomat::fn_traits(t).c_delete},
        &write);
}

inline void somelib::CallbackWrapper::test_inner_conversion(std::function<somelib::diplomat::result<somelib::MyStructContainingAnOption, size_t>()> t) {
    somelib::capi::CallbackWrapper_test_inner_conversion({new decltype(t)(std::move(t)), somelib::diplomat::fn_traits(t).template c_run_callback_result<somelib::MyStructContainingAnOption, size_t, somelib::capi::DiplomatCallback_CallbackWrapper_test_inner_conversion_t_result>, somelib::diplomat::fn_traits(t).c_delete});
}

inline void somelib::CallbackWrapper::test_str_conversion(std::function<somelib::diplomat::result<std::string_view, std::monostate>()> t) {
    somelib::capi::CallbackWrapper_test_str_conversion({new decltype(t)(std::move(t)), somelib::diplomat::fn_traits(t).template c_run_callback_result<std::string_view, std::monostate, somelib::capi::DiplomatCallback_CallbackWrapper_test_str_conversion_t_result>, somelib::diplomat::fn_traits(t).c_delete});
}

inline void somelib::CallbackWrapper::test_slice_conversion(std::function<somelib::diplomat::result<somelib::diplomat::span<const double>, std::monostate>()> t) {
    somelib::capi::CallbackWrapper_test_slice_conversion({new decltype(t)(std::move(t)), somelib::diplomat::fn_traits(t).template c_run_callback_result<somelib::diplomat::span<const double>, std::monostate, somelib::capi::DiplomatCallback_CallbackWrapper_test_slice_conversion_t_result>, somelib::diplomat::fn_traits(t).c_delete});
}

inline void somelib::CallbackWrapper::test_struct_slice_conversion(std::function<somelib::diplomat::result<somelib::diplomat::span<const somelib::PrimitiveStruct>, std::monostate>()> t) {
    somelib::capi::CallbackWrapper_test_struct_slice_conversion({new decltype(t)(std::move(t)), somelib::diplomat::fn_traits(t).template c_run_callback_result<somelib::diplomat::span<const somelib::PrimitiveStruct>, std::monostate, somelib::capi::DiplomatCallback_CallbackWrapper_test_struct_slice_conversion_t_result>, somelib::diplomat::fn_traits(t).c_delete});
}

inline std::string somelib::CallbackWrapper::test_opaque_result_error(std::function<somelib::diplomat::result<std::monostate, const somelib::Opaque&>()> t) {
    std::string output;
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteFromString(output);
    somelib::capi::CallbackWrapper_test_opaque_result_error({new decltype(t)(std::move(t)), somelib::diplomat::fn_traits(t).template c_run_callback_result<std::monostate, const somelib::Opaque&, somelib::capi::DiplomatCallback_CallbackWrapper_test_opaque_result_error_t_result>, somelib::diplomat::fn_traits(t).c_delete},
        &write);
    return output;
}
template<typename W>
inline void somelib::CallbackWrapper::test_opaque_result_error_write(std::function<somelib::diplomat::result<std::monostate, const somelib::Opaque&>()> t, W& writeable) {
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteTrait<W>::Construct(writeable);
    somelib::capi::CallbackWrapper_test_opaque_result_error({new decltype(t)(std::move(t)), somelib::diplomat::fn_traits(t).template c_run_callback_result<std::monostate, const somelib::Opaque&, somelib::capi::DiplomatCallback_CallbackWrapper_test_opaque_result_error_t_result>, somelib::diplomat::fn_traits(t).c_delete},
        &write);
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
