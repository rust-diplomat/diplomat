#ifndef ResultOpaque_HPP
#define ResultOpaque_HPP

#include "ResultOpaque.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "ErrorEnum.hpp"
#include "ErrorStruct.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct ResultOpaque_new_result {union {diplomat::capi::ResultOpaque* ok; diplomat::capi::ErrorEnum err;}; bool is_ok;} ResultOpaque_new_result;
    ResultOpaque_new_result ResultOpaque_new(int32_t i);
    
    typedef struct ResultOpaque_new_failing_foo_result {union {diplomat::capi::ResultOpaque* ok; diplomat::capi::ErrorEnum err;}; bool is_ok;} ResultOpaque_new_failing_foo_result;
    ResultOpaque_new_failing_foo_result ResultOpaque_new_failing_foo(void);
    
    typedef struct ResultOpaque_new_failing_bar_result {union {diplomat::capi::ResultOpaque* ok; diplomat::capi::ErrorEnum err;}; bool is_ok;} ResultOpaque_new_failing_bar_result;
    ResultOpaque_new_failing_bar_result ResultOpaque_new_failing_bar(void);
    
    typedef struct ResultOpaque_new_failing_unit_result {union {diplomat::capi::ResultOpaque* ok; }; bool is_ok;} ResultOpaque_new_failing_unit_result;
    ResultOpaque_new_failing_unit_result ResultOpaque_new_failing_unit(void);
    
    typedef struct ResultOpaque_new_failing_struct_result {union {diplomat::capi::ResultOpaque* ok; diplomat::capi::ErrorStruct err;}; bool is_ok;} ResultOpaque_new_failing_struct_result;
    ResultOpaque_new_failing_struct_result ResultOpaque_new_failing_struct(int32_t i);
    
    typedef struct ResultOpaque_new_in_err_result {union { diplomat::capi::ResultOpaque* err;}; bool is_ok;} ResultOpaque_new_in_err_result;
    ResultOpaque_new_in_err_result ResultOpaque_new_in_err(int32_t i);
    
    typedef struct ResultOpaque_new_int_result {union {int32_t ok; }; bool is_ok;} ResultOpaque_new_int_result;
    ResultOpaque_new_int_result ResultOpaque_new_int(int32_t i);
    
    typedef struct ResultOpaque_new_in_enum_err_result {union {diplomat::capi::ErrorEnum ok; diplomat::capi::ResultOpaque* err;}; bool is_ok;} ResultOpaque_new_in_enum_err_result;
    ResultOpaque_new_in_enum_err_result ResultOpaque_new_in_enum_err(int32_t i);
    
    void ResultOpaque_assert_integer(const diplomat::capi::ResultOpaque* self, int32_t i);
    
    
    void ResultOpaque_destroy(ResultOpaque* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<ResultOpaque>, ErrorEnum> ResultOpaque::new_(int32_t i) {
  auto result = diplomat::capi::ResultOpaque_new(i);
  return result.is_ok ? diplomat::result<std::unique_ptr<ResultOpaque>, ErrorEnum>(diplomat::Ok<std::unique_ptr<ResultOpaque>>(std::unique_ptr<ResultOpaque>(ResultOpaque::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ResultOpaque>, ErrorEnum>(diplomat::Err<ErrorEnum>(ErrorEnum::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ResultOpaque>, ErrorEnum> ResultOpaque::new_failing_foo() {
  auto result = diplomat::capi::ResultOpaque_new_failing_foo();
  return result.is_ok ? diplomat::result<std::unique_ptr<ResultOpaque>, ErrorEnum>(diplomat::Ok<std::unique_ptr<ResultOpaque>>(std::unique_ptr<ResultOpaque>(ResultOpaque::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ResultOpaque>, ErrorEnum>(diplomat::Err<ErrorEnum>(ErrorEnum::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ResultOpaque>, ErrorEnum> ResultOpaque::new_failing_bar() {
  auto result = diplomat::capi::ResultOpaque_new_failing_bar();
  return result.is_ok ? diplomat::result<std::unique_ptr<ResultOpaque>, ErrorEnum>(diplomat::Ok<std::unique_ptr<ResultOpaque>>(std::unique_ptr<ResultOpaque>(ResultOpaque::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ResultOpaque>, ErrorEnum>(diplomat::Err<ErrorEnum>(ErrorEnum::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ResultOpaque>, std::monostate> ResultOpaque::new_failing_unit() {
  auto result = diplomat::capi::ResultOpaque_new_failing_unit();
  return result.is_ok ? diplomat::result<std::unique_ptr<ResultOpaque>, std::monostate>(diplomat::Ok<std::unique_ptr<ResultOpaque>>(std::unique_ptr<ResultOpaque>(ResultOpaque::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ResultOpaque>, std::monostate>(diplomat::Err<std::monostate>());
}

inline diplomat::result<std::unique_ptr<ResultOpaque>, ErrorStruct> ResultOpaque::new_failing_struct(int32_t i) {
  auto result = diplomat::capi::ResultOpaque_new_failing_struct(i);
  return result.is_ok ? diplomat::result<std::unique_ptr<ResultOpaque>, ErrorStruct>(diplomat::Ok<std::unique_ptr<ResultOpaque>>(std::unique_ptr<ResultOpaque>(ResultOpaque::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ResultOpaque>, ErrorStruct>(diplomat::Err<ErrorStruct>(ErrorStruct::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, std::unique_ptr<ResultOpaque>> ResultOpaque::new_in_err(int32_t i) {
  auto result = diplomat::capi::ResultOpaque_new_in_err(i);
  return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<ResultOpaque>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<ResultOpaque>>(diplomat::Err<std::unique_ptr<ResultOpaque>>(std::unique_ptr<ResultOpaque>(ResultOpaque::FromFFI(result.err))));
}

inline diplomat::result<int32_t, std::monostate> ResultOpaque::new_int(int32_t i) {
  auto result = diplomat::capi::ResultOpaque_new_int(i);
  return result.is_ok ? diplomat::result<int32_t, std::monostate>(diplomat::Ok<int32_t>(result.ok)) : diplomat::result<int32_t, std::monostate>(diplomat::Err<std::monostate>());
}

inline diplomat::result<ErrorEnum, std::unique_ptr<ResultOpaque>> ResultOpaque::new_in_enum_err(int32_t i) {
  auto result = diplomat::capi::ResultOpaque_new_in_enum_err(i);
  return result.is_ok ? diplomat::result<ErrorEnum, std::unique_ptr<ResultOpaque>>(diplomat::Ok<ErrorEnum>(ErrorEnum::FromFFI(result.ok))) : diplomat::result<ErrorEnum, std::unique_ptr<ResultOpaque>>(diplomat::Err<std::unique_ptr<ResultOpaque>>(std::unique_ptr<ResultOpaque>(ResultOpaque::FromFFI(result.err))));
}

inline void ResultOpaque::assert_integer(int32_t i) const {
  diplomat::capi::ResultOpaque_assert_integer(this->AsFFI(),
    i);
}

inline const diplomat::capi::ResultOpaque* ResultOpaque::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::ResultOpaque*>(this);
}

inline diplomat::capi::ResultOpaque* ResultOpaque::AsFFI() {
  return reinterpret_cast<diplomat::capi::ResultOpaque*>(this);
}

inline const ResultOpaque* ResultOpaque::FromFFI(const diplomat::capi::ResultOpaque* ptr) {
  return reinterpret_cast<const ResultOpaque*>(ptr);
}

inline ResultOpaque* ResultOpaque::FromFFI(diplomat::capi::ResultOpaque* ptr) {
  return reinterpret_cast<ResultOpaque*>(ptr);
}

inline void ResultOpaque::operator delete(void* ptr) {
  diplomat::capi::ResultOpaque_destroy(reinterpret_cast<diplomat::capi::ResultOpaque*>(ptr));
}


#endif // ResultOpaque_HPP
