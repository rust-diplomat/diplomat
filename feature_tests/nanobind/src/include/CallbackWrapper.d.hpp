#ifndef SOMELIB_CallbackWrapper_D_HPP
#define SOMELIB_CallbackWrapper_D_HPP

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
namespace capi { struct MyString; }
class MyString;
namespace capi { struct Opaque; }
class Opaque;
struct CallbackTestingStruct;
struct MyStruct;
struct PrimitiveStruct;
class FFIError;
} // namespace somelib



namespace somelib {
namespace capi {
    struct CallbackWrapper {
      bool cant_be_empty;
    };

    typedef struct CallbackWrapper_option {union { CallbackWrapper ok; }; bool is_ok; } CallbackWrapper_option;
} // namespace capi
} // namespace


namespace somelib {
struct CallbackWrapper {
    bool cant_be_empty;

  inline static int32_t test_multi_arg_callback_fallible(std::function<somelib::diplomat::result<int32_t, somelib::FFIError>(int32_t)> f, int32_t x);

  inline static int32_t test_no_args_fallible(std::function<somelib::diplomat::result<std::monostate, somelib::FFIError>()> h);

  inline static int32_t test_cb_with_struct_fallible(std::function<somelib::diplomat::result<int32_t, somelib::FFIError>(somelib::CallbackTestingStruct)> f);

  inline static int32_t test_multiple_cb_args_fallible(std::function<somelib::diplomat::result<int32_t, somelib::FFIError>()> f, std::function<somelib::diplomat::result<int32_t, somelib::FFIError>(int32_t)> g);

  inline static int32_t test_str_cb_arg_fallible(std::function<somelib::diplomat::result<int32_t, somelib::FFIError>(std::string_view)> f);

  inline static void test_opaque_cb_arg_fallible(std::function<somelib::diplomat::result<std::monostate, somelib::FFIError>(somelib::MyString&)> cb, somelib::MyString& a);

  inline static std::string test_result_opaque(std::function<somelib::diplomat::result<const somelib::Opaque&, somelib::FFIError>()> t);
  template<typename W>
  inline static void test_result_opaque_write(std::function<somelib::diplomat::result<const somelib::Opaque&, somelib::FFIError>()> t, W& writeable_output);

  inline static void test_str_conversion(std::function<somelib::diplomat::result<std::string_view, somelib::FFIError>()> t);

  inline static void test_slice_conversion(std::function<somelib::diplomat::result<somelib::diplomat::span<const double>, somelib::FFIError>()> t);

  inline static void test_result_option_struct_conversion(std::function<somelib::diplomat::result<std::optional<somelib::MyStruct>, somelib::FFIError>()> t);

  inline static void test_struct_slice_conversion(std::function<somelib::diplomat::result<somelib::diplomat::span<const somelib::PrimitiveStruct>, somelib::FFIError>()> t);

  inline static somelib::diplomat::result<std::monostate, somelib::FFIError> test_ffi_error(std::function<somelib::diplomat::result<std::monostate, somelib::FFIError>()> t);

  inline static somelib::FFIError test_ffi_error_as_ok(std::function<somelib::diplomat::result<std::monostate, somelib::FFIError>()> t);

    inline somelib::capi::CallbackWrapper AsFFI() const;
    inline static somelib::CallbackWrapper FromFFI(somelib::capi::CallbackWrapper c_struct);
};

} // namespace
#endif // SOMELIB_CallbackWrapper_D_HPP
