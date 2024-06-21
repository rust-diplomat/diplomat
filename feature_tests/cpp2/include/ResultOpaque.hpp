#ifndef ResultOpaque_HPP
#define ResultOpaque_HPP

#include "ResultOpaque.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ErrorEnum.hpp"
#include "ErrorStruct.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ResultOpaque_new_result {union {ResultOpaque* ok; ErrorEnum err;}; bool is_ok;} ResultOpaque_new_result;
    ResultOpaque_new_result ResultOpaque_new(int32_t i);
    
    typedef struct ResultOpaque_new_failing_foo_result {union {ResultOpaque* ok; ErrorEnum err;}; bool is_ok;} ResultOpaque_new_failing_foo_result;
    ResultOpaque_new_failing_foo_result ResultOpaque_new_failing_foo();
    
    typedef struct ResultOpaque_new_failing_bar_result {union {ResultOpaque* ok; ErrorEnum err;}; bool is_ok;} ResultOpaque_new_failing_bar_result;
    ResultOpaque_new_failing_bar_result ResultOpaque_new_failing_bar();
    
    typedef struct ResultOpaque_new_failing_unit_result {union {ResultOpaque* ok; }; bool is_ok;} ResultOpaque_new_failing_unit_result;
    ResultOpaque_new_failing_unit_result ResultOpaque_new_failing_unit();
    
    typedef struct ResultOpaque_new_failing_struct_result {union {ResultOpaque* ok; ErrorStruct err;}; bool is_ok;} ResultOpaque_new_failing_struct_result;
    ResultOpaque_new_failing_struct_result ResultOpaque_new_failing_struct(int32_t i);
    
    typedef struct ResultOpaque_new_in_err_result {union { ResultOpaque* err;}; bool is_ok;} ResultOpaque_new_in_err_result;
    ResultOpaque_new_in_err_result ResultOpaque_new_in_err(int32_t i);
    
    typedef struct ResultOpaque_new_int_result {union {int32_t ok; }; bool is_ok;} ResultOpaque_new_int_result;
    ResultOpaque_new_int_result ResultOpaque_new_int(int32_t i);
    
    typedef struct ResultOpaque_new_in_enum_err_result {union {ErrorEnum ok; ResultOpaque* err;}; bool is_ok;} ResultOpaque_new_in_enum_err_result;
    ResultOpaque_new_in_enum_err_result ResultOpaque_new_in_enum_err(int32_t i);
    
    void ResultOpaque_assert_integer(const ResultOpaque* self, int32_t i);
    
    
    void ResultOpaque_destroy(ResultOpaque* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ResultOpaque>, ErrorEnum> ResultOpaque::new_(int32_t i) {
  auto result = capi::ResultOpaque_new(i);
  return result.is_ok ? diplomat::result<std::unique_ptr<ResultOpaque>, ErrorEnum>(diplomat::Ok<std::unique_ptr<ResultOpaque>>(std::unique_ptr<ResultOpaque>(ResultOpaque::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ResultOpaque>, ErrorEnum>(diplomat::Err<ErrorEnum>(ErrorEnum::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ResultOpaque>, ErrorEnum> ResultOpaque::new_failing_foo() {
  auto result = capi::ResultOpaque_new_failing_foo();
  return result.is_ok ? diplomat::result<std::unique_ptr<ResultOpaque>, ErrorEnum>(diplomat::Ok<std::unique_ptr<ResultOpaque>>(std::unique_ptr<ResultOpaque>(ResultOpaque::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ResultOpaque>, ErrorEnum>(diplomat::Err<ErrorEnum>(ErrorEnum::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ResultOpaque>, ErrorEnum> ResultOpaque::new_failing_bar() {
  auto result = capi::ResultOpaque_new_failing_bar();
  return result.is_ok ? diplomat::result<std::unique_ptr<ResultOpaque>, ErrorEnum>(diplomat::Ok<std::unique_ptr<ResultOpaque>>(std::unique_ptr<ResultOpaque>(ResultOpaque::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ResultOpaque>, ErrorEnum>(diplomat::Err<ErrorEnum>(ErrorEnum::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ResultOpaque>, std::monostate> ResultOpaque::new_failing_unit() {
  auto result = capi::ResultOpaque_new_failing_unit();
  return result.is_ok ? diplomat::result<std::unique_ptr<ResultOpaque>, std::monostate>(diplomat::Ok<std::unique_ptr<ResultOpaque>>(std::unique_ptr<ResultOpaque>(ResultOpaque::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ResultOpaque>, std::monostate>(diplomat::Err<std::monostate>());
}

inline diplomat::result<std::unique_ptr<ResultOpaque>, ErrorStruct> ResultOpaque::new_failing_struct(int32_t i) {
  auto result = capi::ResultOpaque_new_failing_struct(i);
  return result.is_ok ? diplomat::result<std::unique_ptr<ResultOpaque>, ErrorStruct>(diplomat::Ok<std::unique_ptr<ResultOpaque>>(std::unique_ptr<ResultOpaque>(ResultOpaque::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ResultOpaque>, ErrorStruct>(diplomat::Err<ErrorStruct>(ErrorStruct::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, std::unique_ptr<ResultOpaque>> ResultOpaque::new_in_err(int32_t i) {
  auto result = capi::ResultOpaque_new_in_err(i);
  return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<ResultOpaque>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<ResultOpaque>>(diplomat::Err<std::unique_ptr<ResultOpaque>>(std::unique_ptr<ResultOpaque>(ResultOpaque::FromFFI(result.err))));
}

inline diplomat::result<int32_t, std::monostate> ResultOpaque::new_int(int32_t i) {
  auto result = capi::ResultOpaque_new_int(i);
  return result.is_ok ? diplomat::result<int32_t, std::monostate>(diplomat::Ok<int32_t>(result.ok)) : diplomat::result<int32_t, std::monostate>(diplomat::Err<std::monostate>());
}

inline diplomat::result<ErrorEnum, std::unique_ptr<ResultOpaque>> ResultOpaque::new_in_enum_err(int32_t i) {
  auto result = capi::ResultOpaque_new_in_enum_err(i);
  return result.is_ok ? diplomat::result<ErrorEnum, std::unique_ptr<ResultOpaque>>(diplomat::Ok<ErrorEnum>(ErrorEnum::FromFFI(result.ok))) : diplomat::result<ErrorEnum, std::unique_ptr<ResultOpaque>>(diplomat::Err<std::unique_ptr<ResultOpaque>>(std::unique_ptr<ResultOpaque>(ResultOpaque::FromFFI(result.err))));
}

inline void ResultOpaque::assert_integer(int32_t i) const {
  capi::ResultOpaque_assert_integer(this->AsFFI(),
    i);
}

inline const capi::ResultOpaque* ResultOpaque::AsFFI() const {
  return reinterpret_cast<const capi::ResultOpaque*>(this);
}

inline capi::ResultOpaque* ResultOpaque::AsFFI() {
  return reinterpret_cast<capi::ResultOpaque*>(this);
}

inline const ResultOpaque* ResultOpaque::FromFFI(const capi::ResultOpaque* ptr) {
  return reinterpret_cast<const ResultOpaque*>(ptr);
}

inline ResultOpaque* ResultOpaque::FromFFI(capi::ResultOpaque* ptr) {
  return reinterpret_cast<ResultOpaque*>(ptr);
}

inline void ResultOpaque::operator delete(void* ptr) {
  capi::ResultOpaque_destroy(reinterpret_cast<capi::ResultOpaque*>(ptr));
}


#endif // ResultOpaque_HPP
