#ifndef ResultOpaque_HPP
#define ResultOpaque_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ResultOpaque.d.hpp"
#include "ResultOpaque.h"


inline DiplomatResult<std::unique_ptr<ResultOpaque>, ErrorEnum> ResultOpaque::new_(int32_t i) {
  auto result = capi::ResultOpaque_new(i);
  return std::unique_ptr(ResultOpaque::FromFFI(result));
}

inline DiplomatResult<std::unique_ptr<ResultOpaque>, ErrorEnum> ResultOpaque::new_failing_foo() {
  auto result = capi::ResultOpaque_new_failing_foo();
  return std::unique_ptr(ResultOpaque::FromFFI(result));
}

inline DiplomatResult<std::unique_ptr<ResultOpaque>, ErrorEnum> ResultOpaque::new_failing_bar() {
  auto result = capi::ResultOpaque_new_failing_bar();
  return std::unique_ptr(ResultOpaque::FromFFI(result));
}

inline DiplomatResult<std::unique_ptr<ResultOpaque>, std::monostate> ResultOpaque::new_failing_unit() {
  auto result = capi::ResultOpaque_new_failing_unit();
  return std::unique_ptr(ResultOpaque::FromFFI(result));
}

inline DiplomatResult<std::unique_ptr<ResultOpaque>, ErrorStruct> ResultOpaque::new_failing_struct(int32_t i) {
  auto result = capi::ResultOpaque_new_failing_struct(i);
  return std::unique_ptr(ResultOpaque::FromFFI(result));
}

inline DiplomatResult<std::monostate, std::unique_ptr<ResultOpaque>> ResultOpaque::new_in_err(int32_t i) {
  capi::ResultOpaque_new_in_err(i);
}

inline DiplomatResult<ErrorEnum, std::unique_ptr<ResultOpaque>> ResultOpaque::new_in_enum_err(int32_t i) {
  auto result = capi::ResultOpaque_new_in_enum_err(i);
  return ResultOpaque::FromFFI(result);
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

inline ResultOpaque::~ResultOpaque() {
  capi::ResultOpaque_destroy(AsFFI());
}


#endif // ResultOpaque_HPP
