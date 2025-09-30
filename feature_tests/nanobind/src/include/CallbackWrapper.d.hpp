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
struct MyStructContainingAnOption;
struct PrimitiveStruct;
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

  inline static int32_t test_multi_arg_callback(std::function<int32_t(int32_t)> f, int32_t x);

  inline static int32_t test_no_args(std::function<void()> h);

  inline static int32_t test_cb_with_struct(std::function<int32_t(somelib::CallbackTestingStruct)> f);

  inline static int32_t test_multiple_cb_args(std::function<int32_t()> f, std::function<int32_t(int32_t)> g);

  inline static int32_t test_str_cb_arg(std::function<int32_t(std::string_view)> f);

  inline static void test_opaque_cb_arg(std::function<void(somelib::MyString&)> cb, somelib::MyString& a);

  inline static void test_slice_cb_arg(somelib::diplomat::span<const uint8_t> arg, std::function<void(somelib::diplomat::span<const uint8_t>)> f);

  inline static void test_result_output(std::function<somelib::diplomat::result<std::monostate, std::monostate>()> t);

  inline static void test_result_usize_output(std::function<somelib::diplomat::result<size_t, std::monostate>()> t);

  inline static void test_option_output(std::function<std::optional<std::monostate>()> t);

  inline static void test_diplomat_option_output(std::function<std::optional<uint32_t>()> t);

  inline static std::string test_option_opaque(std::function<const somelib::Opaque*()> t);
  template<typename W>
  inline static void test_option_opaque_write(std::function<const somelib::Opaque*()> t, W& writeable_output);

  inline static void test_diplomat_result(std::function<somelib::diplomat::result<size_t, size_t>()> t);

  inline static std::string test_result_opaque(std::function<somelib::diplomat::result<const somelib::Opaque&, std::monostate>()> t);
  template<typename W>
  inline static void test_result_opaque_write(std::function<somelib::diplomat::result<const somelib::Opaque&, std::monostate>()> t, W& writeable_output);

  inline static void test_inner_conversion(std::function<somelib::diplomat::result<somelib::MyStructContainingAnOption, size_t>()> t);

  inline static void test_str_conversion(std::function<somelib::diplomat::result<std::string_view, std::monostate>()> t);

  inline static void test_slice_conversion(std::function<somelib::diplomat::result<somelib::diplomat::span<const double>, std::monostate>()> t);

  inline static void test_struct_slice_conversion(std::function<somelib::diplomat::result<somelib::diplomat::span<const somelib::PrimitiveStruct>, std::monostate>()> t);

  inline static std::string test_opaque_result_error(std::function<somelib::diplomat::result<std::monostate, const somelib::Opaque&>()> t);
  template<typename W>
  inline static void test_opaque_result_error_write(std::function<somelib::diplomat::result<std::monostate, const somelib::Opaque&>()> t, W& writeable_output);

    inline somelib::capi::CallbackWrapper AsFFI() const;
    inline static somelib::CallbackWrapper FromFFI(somelib::capi::CallbackWrapper c_struct);
};

} // namespace
#endif // SOMELIB_CallbackWrapper_D_HPP
