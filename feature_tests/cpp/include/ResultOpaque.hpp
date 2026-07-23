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

    typedef struct ResultOpaque_give_self_result {union { const somelib::capi::ResultOpaque* err;}; bool is_ok;} ResultOpaque_give_self_result;
    ResultOpaque_give_self_result ResultOpaque_give_self(const somelib::capi::ResultOpaque* self);

    somelib::capi::ResultOpaque* ResultOpaque_takes_str(somelib::capi::ResultOpaque* self, somelib::diplomat::capi::DiplomatStringView _v);

    typedef struct ResultOpaque_stringify_error_result {union { const somelib::capi::ResultOpaque* err;}; bool is_ok;} ResultOpaque_stringify_error_result;
    ResultOpaque_stringify_error_result ResultOpaque_stringify_error(const somelib::capi::ResultOpaque* self, somelib::diplomat::capi::DiplomatWrite* write);

    void ResultOpaque_assert_integer(const somelib::capi::ResultOpaque* self, int32_t i);

    void ResultOpaque_destroy(ResultOpaque* self);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::diplomat::result<somelib::ResultOpaque, somelib::ErrorEnum> somelib::ResultOpaque::new_(int32_t i) {
    auto result = somelib::capi::ResultOpaque_new(i);
    return result.is_ok ? somelib::diplomat::result<somelib::ResultOpaque, somelib::ErrorEnum>(somelib::diplomat::Ok<somelib::ResultOpaque>(somelib::ResultOpaque::FromFFI(result.ok))) : somelib::diplomat::result<somelib::ResultOpaque, somelib::ErrorEnum>(somelib::diplomat::Err<somelib::ErrorEnum>(somelib::ErrorEnum::FromFFI(result.err)));
}

inline somelib::diplomat::result<somelib::ResultOpaque, somelib::ErrorEnum> somelib::ResultOpaque::new_failing_foo() {
    auto result = somelib::capi::ResultOpaque_new_failing_foo();
    return result.is_ok ? somelib::diplomat::result<somelib::ResultOpaque, somelib::ErrorEnum>(somelib::diplomat::Ok<somelib::ResultOpaque>(somelib::ResultOpaque::FromFFI(result.ok))) : somelib::diplomat::result<somelib::ResultOpaque, somelib::ErrorEnum>(somelib::diplomat::Err<somelib::ErrorEnum>(somelib::ErrorEnum::FromFFI(result.err)));
}

inline somelib::diplomat::result<somelib::ResultOpaque, somelib::ErrorEnum> somelib::ResultOpaque::new_failing_bar() {
    auto result = somelib::capi::ResultOpaque_new_failing_bar();
    return result.is_ok ? somelib::diplomat::result<somelib::ResultOpaque, somelib::ErrorEnum>(somelib::diplomat::Ok<somelib::ResultOpaque>(somelib::ResultOpaque::FromFFI(result.ok))) : somelib::diplomat::result<somelib::ResultOpaque, somelib::ErrorEnum>(somelib::diplomat::Err<somelib::ErrorEnum>(somelib::ErrorEnum::FromFFI(result.err)));
}

inline somelib::diplomat::result<somelib::ResultOpaque, std::monostate> somelib::ResultOpaque::new_failing_unit() {
    auto result = somelib::capi::ResultOpaque_new_failing_unit();
    return result.is_ok ? somelib::diplomat::result<somelib::ResultOpaque, std::monostate>(somelib::diplomat::Ok<somelib::ResultOpaque>(somelib::ResultOpaque::FromFFI(result.ok))) : somelib::diplomat::result<somelib::ResultOpaque, std::monostate>(somelib::diplomat::Err<std::monostate>());
}

inline somelib::diplomat::result<somelib::ResultOpaque, somelib::ErrorStruct> somelib::ResultOpaque::new_failing_struct(int32_t i) {
    auto result = somelib::capi::ResultOpaque_new_failing_struct(i);
    return result.is_ok ? somelib::diplomat::result<somelib::ResultOpaque, somelib::ErrorStruct>(somelib::diplomat::Ok<somelib::ResultOpaque>(somelib::ResultOpaque::FromFFI(result.ok))) : somelib::diplomat::result<somelib::ResultOpaque, somelib::ErrorStruct>(somelib::diplomat::Err<somelib::ErrorStruct>(somelib::ErrorStruct::FromFFI(result.err)));
}

inline somelib::diplomat::result<std::monostate, somelib::ResultOpaque> somelib::ResultOpaque::new_in_err(int32_t i) {
    auto result = somelib::capi::ResultOpaque_new_in_err(i);
    return result.is_ok ? somelib::diplomat::result<std::monostate, somelib::ResultOpaque>(somelib::diplomat::Ok<std::monostate>()) : somelib::diplomat::result<std::monostate, somelib::ResultOpaque>(somelib::diplomat::Err<somelib::ResultOpaque>(somelib::ResultOpaque::FromFFI(result.err)));
}

inline somelib::diplomat::result<int32_t, std::monostate> somelib::ResultOpaque::new_int(int32_t i) {
    auto result = somelib::capi::ResultOpaque_new_int(i);
    return result.is_ok ? somelib::diplomat::result<int32_t, std::monostate>(somelib::diplomat::Ok<int32_t>(result.ok)) : somelib::diplomat::result<int32_t, std::monostate>(somelib::diplomat::Err<std::monostate>());
}

inline somelib::diplomat::result<somelib::ErrorEnum, somelib::ResultOpaque> somelib::ResultOpaque::new_in_enum_err(int32_t i) {
    auto result = somelib::capi::ResultOpaque_new_in_enum_err(i);
    return result.is_ok ? somelib::diplomat::result<somelib::ErrorEnum, somelib::ResultOpaque>(somelib::diplomat::Ok<somelib::ErrorEnum>(somelib::ErrorEnum::FromFFI(result.ok))) : somelib::diplomat::result<somelib::ErrorEnum, somelib::ResultOpaque>(somelib::diplomat::Err<somelib::ResultOpaque>(somelib::ResultOpaque::FromFFI(result.err)));
}

inline somelib::diplomat::result<std::monostate, somelib::ResultOpaqueRef> somelib::ResultOpaque::give_self() const DIPLOMAT_LIFETIME_BOUND {
    auto result = somelib::capi::ResultOpaque_give_self(this->AsFFI());
    return result.is_ok ? somelib::diplomat::result<std::monostate, somelib::ResultOpaqueRef>(somelib::diplomat::Ok<std::monostate>()) : somelib::diplomat::result<std::monostate, somelib::ResultOpaqueRef>(somelib::diplomat::Err<somelib::ResultOpaqueRef>(somelib::ResultOpaqueRef::FromFFI(result.err)));
}

inline somelib::diplomat::result<somelib::ResultOpaqueRefMut, somelib::diplomat::Utf8Error> somelib::ResultOpaque::takes_str(std::string_view _v) DIPLOMAT_LIFETIME_BOUND {
    if (!somelib::diplomat::capi::diplomat_is_str(_v.data(), _v.size())) {
    return somelib::diplomat::Err<somelib::diplomat::Utf8Error>();
  }
    auto result = somelib::capi::ResultOpaque_takes_str(this->AsFFI(),
        {_v.data(), _v.size()});
    return somelib::diplomat::Ok<somelib::ResultOpaqueRefMut>(somelib::ResultOpaqueRefMut::FromFFI(result));
}

inline somelib::diplomat::result<std::string, somelib::ResultOpaqueRef> somelib::ResultOpaque::stringify_error() const DIPLOMAT_LIFETIME_BOUND {
    std::string output;
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteFromString(output);
    auto result = somelib::capi::ResultOpaque_stringify_error(this->AsFFI(),
        &write);
    return result.is_ok ? somelib::diplomat::result<std::string, somelib::ResultOpaqueRef>(somelib::diplomat::Ok<std::string>(std::move(output))) : somelib::diplomat::result<std::string, somelib::ResultOpaqueRef>(somelib::diplomat::Err<somelib::ResultOpaqueRef>(somelib::ResultOpaqueRef::FromFFI(result.err)));
}
template<typename W>
inline somelib::diplomat::result<std::monostate, somelib::ResultOpaqueRef> somelib::ResultOpaque::stringify_error_write(W& writeable) const DIPLOMAT_LIFETIME_BOUND {
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = somelib::capi::ResultOpaque_stringify_error(this->AsFFI(),
        &write);
    return result.is_ok ? somelib::diplomat::result<std::monostate, somelib::ResultOpaqueRef>(somelib::diplomat::Ok<std::monostate>()) : somelib::diplomat::result<std::monostate, somelib::ResultOpaqueRef>(somelib::diplomat::Err<somelib::ResultOpaqueRef>(somelib::ResultOpaqueRef::FromFFI(result.err)));
}

inline void somelib::ResultOpaque::assert_integer(int32_t i) const {
    somelib::capi::ResultOpaque_assert_integer(this->AsFFI(),
        i);
}


#endif // SOMELIB_ResultOpaque_HPP
