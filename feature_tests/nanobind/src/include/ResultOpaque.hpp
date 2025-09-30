#ifndef SOMELIB_ResultOpaque_HPP
#define SOMELIB_ResultOpaque_HPP

#include "ResultOpaque.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "ErrorEnum.hpp"
#include "ErrorStruct.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    typedef struct ResultOpaque_new_result {union {somelib::capi::ResultOpaque* ok; somelib::capi::ErrorEnum err;}; bool is_ok;} ResultOpaque_new_result;
    ResultOpaque_new_result ResultOpaque_new(int32_t i);

    typedef struct ResultOpaque_new_failing_foo_result {union {somelib::capi::ResultOpaque* ok; somelib::capi::ErrorEnum err;}; bool is_ok;} ResultOpaque_new_failing_foo_result;
    ResultOpaque_new_failing_foo_result ResultOpaque_new_failing_foo(void);

    typedef struct ResultOpaque_new_failing_bar_result {union {somelib::capi::ResultOpaque* ok; somelib::capi::ErrorEnum err;}; bool is_ok;} ResultOpaque_new_failing_bar_result;
    ResultOpaque_new_failing_bar_result ResultOpaque_new_failing_bar(void);

    typedef struct ResultOpaque_new_failing_unit_result {union {somelib::capi::ResultOpaque* ok; }; bool is_ok;} ResultOpaque_new_failing_unit_result;
    ResultOpaque_new_failing_unit_result ResultOpaque_new_failing_unit(void);

    typedef struct ResultOpaque_new_failing_struct_result {union {somelib::capi::ResultOpaque* ok; somelib::capi::ErrorStruct err;}; bool is_ok;} ResultOpaque_new_failing_struct_result;
    ResultOpaque_new_failing_struct_result ResultOpaque_new_failing_struct(int32_t i);

    typedef struct ResultOpaque_new_in_err_result {union { somelib::capi::ResultOpaque* err;}; bool is_ok;} ResultOpaque_new_in_err_result;
    ResultOpaque_new_in_err_result ResultOpaque_new_in_err(int32_t i);

    typedef struct ResultOpaque_new_int_result {union {int32_t ok; }; bool is_ok;} ResultOpaque_new_int_result;
    ResultOpaque_new_int_result ResultOpaque_new_int(int32_t i);

    typedef struct ResultOpaque_new_in_enum_err_result {union {somelib::capi::ErrorEnum ok; somelib::capi::ResultOpaque* err;}; bool is_ok;} ResultOpaque_new_in_enum_err_result;
    ResultOpaque_new_in_enum_err_result ResultOpaque_new_in_enum_err(int32_t i);

    somelib::capi::ResultOpaque* ResultOpaque_takes_str(somelib::capi::ResultOpaque* self, somelib::diplomat::capi::DiplomatStringView _v);

    void ResultOpaque_assert_integer(const somelib::capi::ResultOpaque* self, int32_t i);

    void ResultOpaque_destroy(ResultOpaque* self);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::diplomat::result<std::unique_ptr<somelib::ResultOpaque>, somelib::ErrorEnum> somelib::ResultOpaque::new_(int32_t i) {
    auto result = somelib::capi::ResultOpaque_new(i);
    return result.is_ok ? somelib::diplomat::result<std::unique_ptr<somelib::ResultOpaque>, somelib::ErrorEnum>(somelib::diplomat::Ok<std::unique_ptr<somelib::ResultOpaque>>(std::unique_ptr<somelib::ResultOpaque>(somelib::ResultOpaque::FromFFI(result.ok)))) : somelib::diplomat::result<std::unique_ptr<somelib::ResultOpaque>, somelib::ErrorEnum>(somelib::diplomat::Err<somelib::ErrorEnum>(somelib::ErrorEnum::FromFFI(result.err)));
}

inline somelib::diplomat::result<std::unique_ptr<somelib::ResultOpaque>, somelib::ErrorEnum> somelib::ResultOpaque::new_failing_foo() {
    auto result = somelib::capi::ResultOpaque_new_failing_foo();
    return result.is_ok ? somelib::diplomat::result<std::unique_ptr<somelib::ResultOpaque>, somelib::ErrorEnum>(somelib::diplomat::Ok<std::unique_ptr<somelib::ResultOpaque>>(std::unique_ptr<somelib::ResultOpaque>(somelib::ResultOpaque::FromFFI(result.ok)))) : somelib::diplomat::result<std::unique_ptr<somelib::ResultOpaque>, somelib::ErrorEnum>(somelib::diplomat::Err<somelib::ErrorEnum>(somelib::ErrorEnum::FromFFI(result.err)));
}

inline somelib::diplomat::result<std::unique_ptr<somelib::ResultOpaque>, somelib::ErrorEnum> somelib::ResultOpaque::new_failing_bar() {
    auto result = somelib::capi::ResultOpaque_new_failing_bar();
    return result.is_ok ? somelib::diplomat::result<std::unique_ptr<somelib::ResultOpaque>, somelib::ErrorEnum>(somelib::diplomat::Ok<std::unique_ptr<somelib::ResultOpaque>>(std::unique_ptr<somelib::ResultOpaque>(somelib::ResultOpaque::FromFFI(result.ok)))) : somelib::diplomat::result<std::unique_ptr<somelib::ResultOpaque>, somelib::ErrorEnum>(somelib::diplomat::Err<somelib::ErrorEnum>(somelib::ErrorEnum::FromFFI(result.err)));
}

inline somelib::diplomat::result<std::unique_ptr<somelib::ResultOpaque>, std::monostate> somelib::ResultOpaque::new_failing_unit() {
    auto result = somelib::capi::ResultOpaque_new_failing_unit();
    return result.is_ok ? somelib::diplomat::result<std::unique_ptr<somelib::ResultOpaque>, std::monostate>(somelib::diplomat::Ok<std::unique_ptr<somelib::ResultOpaque>>(std::unique_ptr<somelib::ResultOpaque>(somelib::ResultOpaque::FromFFI(result.ok)))) : somelib::diplomat::result<std::unique_ptr<somelib::ResultOpaque>, std::monostate>(somelib::diplomat::Err<std::monostate>());
}

inline somelib::diplomat::result<std::unique_ptr<somelib::ResultOpaque>, somelib::ErrorStruct> somelib::ResultOpaque::new_failing_struct(int32_t i) {
    auto result = somelib::capi::ResultOpaque_new_failing_struct(i);
    return result.is_ok ? somelib::diplomat::result<std::unique_ptr<somelib::ResultOpaque>, somelib::ErrorStruct>(somelib::diplomat::Ok<std::unique_ptr<somelib::ResultOpaque>>(std::unique_ptr<somelib::ResultOpaque>(somelib::ResultOpaque::FromFFI(result.ok)))) : somelib::diplomat::result<std::unique_ptr<somelib::ResultOpaque>, somelib::ErrorStruct>(somelib::diplomat::Err<somelib::ErrorStruct>(somelib::ErrorStruct::FromFFI(result.err)));
}

inline somelib::diplomat::result<std::monostate, std::unique_ptr<somelib::ResultOpaque>> somelib::ResultOpaque::new_in_err(int32_t i) {
    auto result = somelib::capi::ResultOpaque_new_in_err(i);
    return result.is_ok ? somelib::diplomat::result<std::monostate, std::unique_ptr<somelib::ResultOpaque>>(somelib::diplomat::Ok<std::monostate>()) : somelib::diplomat::result<std::monostate, std::unique_ptr<somelib::ResultOpaque>>(somelib::diplomat::Err<std::unique_ptr<somelib::ResultOpaque>>(std::unique_ptr<somelib::ResultOpaque>(somelib::ResultOpaque::FromFFI(result.err))));
}

inline somelib::diplomat::result<int32_t, std::monostate> somelib::ResultOpaque::new_int(int32_t i) {
    auto result = somelib::capi::ResultOpaque_new_int(i);
    return result.is_ok ? somelib::diplomat::result<int32_t, std::monostate>(somelib::diplomat::Ok<int32_t>(result.ok)) : somelib::diplomat::result<int32_t, std::monostate>(somelib::diplomat::Err<std::monostate>());
}

inline somelib::diplomat::result<somelib::ErrorEnum, std::unique_ptr<somelib::ResultOpaque>> somelib::ResultOpaque::new_in_enum_err(int32_t i) {
    auto result = somelib::capi::ResultOpaque_new_in_enum_err(i);
    return result.is_ok ? somelib::diplomat::result<somelib::ErrorEnum, std::unique_ptr<somelib::ResultOpaque>>(somelib::diplomat::Ok<somelib::ErrorEnum>(somelib::ErrorEnum::FromFFI(result.ok))) : somelib::diplomat::result<somelib::ErrorEnum, std::unique_ptr<somelib::ResultOpaque>>(somelib::diplomat::Err<std::unique_ptr<somelib::ResultOpaque>>(std::unique_ptr<somelib::ResultOpaque>(somelib::ResultOpaque::FromFFI(result.err))));
}

inline somelib::diplomat::result<somelib::ResultOpaque&, somelib::diplomat::Utf8Error> somelib::ResultOpaque::takes_str(std::string_view _v) {
    if (!somelib::diplomat::capi::diplomat_is_str(_v.data(), _v.size())) {
    return somelib::diplomat::Err<somelib::diplomat::Utf8Error>();
  }
    auto result = somelib::capi::ResultOpaque_takes_str(this->AsFFI(),
        {_v.data(), _v.size()});
    return somelib::diplomat::Ok<somelib::ResultOpaque&>(*somelib::ResultOpaque::FromFFI(result));
}

inline void somelib::ResultOpaque::assert_integer(int32_t i) const {
    somelib::capi::ResultOpaque_assert_integer(this->AsFFI(),
        i);
}

inline const somelib::capi::ResultOpaque* somelib::ResultOpaque::AsFFI() const {
    return reinterpret_cast<const somelib::capi::ResultOpaque*>(this);
}

inline somelib::capi::ResultOpaque* somelib::ResultOpaque::AsFFI() {
    return reinterpret_cast<somelib::capi::ResultOpaque*>(this);
}

inline const somelib::ResultOpaque* somelib::ResultOpaque::FromFFI(const somelib::capi::ResultOpaque* ptr) {
    return reinterpret_cast<const somelib::ResultOpaque*>(ptr);
}

inline somelib::ResultOpaque* somelib::ResultOpaque::FromFFI(somelib::capi::ResultOpaque* ptr) {
    return reinterpret_cast<somelib::ResultOpaque*>(ptr);
}

inline void somelib::ResultOpaque::operator delete(void* ptr) {
    somelib::capi::ResultOpaque_destroy(reinterpret_cast<somelib::capi::ResultOpaque*>(ptr));
}


#endif // SOMELIB_ResultOpaque_HPP
