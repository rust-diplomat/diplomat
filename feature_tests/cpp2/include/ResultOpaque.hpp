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
#include "ResultOpaque.h"


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
