#ifndef ResultOpaque_HPP
#define ResultOpaque_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ResultOpaque.h"

class ResultOpaque;
#include "ErrorEnum.hpp"
struct ErrorStruct;

/**
 * A destruction policy for using ResultOpaque with std::unique_ptr.
 */
struct ResultOpaqueDeleter {
  void operator()(capi::ResultOpaque* l) const noexcept {
    capi::ResultOpaque_destroy(l);
  }
};
class ResultOpaque {
 public:
  static diplomat::result<ResultOpaque, ErrorEnum> new_(int32_t i);
  static diplomat::result<ResultOpaque, ErrorEnum> new_failing_foo();
  static diplomat::result<ResultOpaque, ErrorEnum> new_failing_bar();
  static diplomat::result<ResultOpaque, std::monostate> new_failing_unit();
  static diplomat::result<ResultOpaque, ErrorStruct> new_failing_struct(int32_t i);
  static diplomat::result<std::monostate, ResultOpaque> new_in_err(int32_t i);
  static diplomat::result<int32_t, std::monostate> new_int(int32_t i);
  static diplomat::result<ErrorEnum, ResultOpaque> new_in_enum_err(int32_t i);
  void assert_integer(int32_t i) const;
  inline const capi::ResultOpaque* AsFFI() const { return this->inner.get(); }
  inline capi::ResultOpaque* AsFFIMut() { return this->inner.get(); }
  inline explicit ResultOpaque(capi::ResultOpaque* i) : inner(i) {}
  ResultOpaque() = default;
  ResultOpaque(ResultOpaque&&) noexcept = default;
  ResultOpaque& operator=(ResultOpaque&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ResultOpaque, ResultOpaqueDeleter> inner;
};

#include "ErrorStruct.hpp"

inline diplomat::result<ResultOpaque, ErrorEnum> ResultOpaque::new_(int32_t i) {
  auto diplomat_result_raw_out_value = capi::ResultOpaque_new(i);
  diplomat::result<ResultOpaque, ErrorEnum> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ResultOpaque>(ResultOpaque(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err<ErrorEnum>(static_cast<ErrorEnum>(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ResultOpaque, ErrorEnum> ResultOpaque::new_failing_foo() {
  auto diplomat_result_raw_out_value = capi::ResultOpaque_new_failing_foo();
  diplomat::result<ResultOpaque, ErrorEnum> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ResultOpaque>(ResultOpaque(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err<ErrorEnum>(static_cast<ErrorEnum>(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ResultOpaque, ErrorEnum> ResultOpaque::new_failing_bar() {
  auto diplomat_result_raw_out_value = capi::ResultOpaque_new_failing_bar();
  diplomat::result<ResultOpaque, ErrorEnum> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ResultOpaque>(ResultOpaque(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err<ErrorEnum>(static_cast<ErrorEnum>(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ResultOpaque, std::monostate> ResultOpaque::new_failing_unit() {
  auto diplomat_result_raw_out_value = capi::ResultOpaque_new_failing_unit();
  diplomat::result<ResultOpaque, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ResultOpaque>(ResultOpaque(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err(std::monostate());
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ResultOpaque, ErrorStruct> ResultOpaque::new_failing_struct(int32_t i) {
  auto diplomat_result_raw_out_value = capi::ResultOpaque_new_failing_struct(i);
  diplomat::result<ResultOpaque, ErrorStruct> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ResultOpaque>(ResultOpaque(diplomat_result_raw_out_value.ok));
  } else {
  capi::ErrorStruct diplomat_raw_struct_out_value = diplomat_result_raw_out_value.err;
    diplomat_result_out_value = diplomat::Err<ErrorStruct>(ErrorStruct{ .i = std::move(diplomat_raw_struct_out_value.i), .j = std::move(diplomat_raw_struct_out_value.j) });
  }
  return diplomat_result_out_value;
}
inline diplomat::result<std::monostate, ResultOpaque> ResultOpaque::new_in_err(int32_t i) {
  auto diplomat_result_raw_out_value = capi::ResultOpaque_new_in_err(i);
  diplomat::result<std::monostate, ResultOpaque> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ResultOpaque>(ResultOpaque(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<int32_t, std::monostate> ResultOpaque::new_int(int32_t i) {
  auto diplomat_result_raw_out_value = capi::ResultOpaque_new_int(i);
  diplomat::result<int32_t, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<int32_t>(diplomat_result_raw_out_value.ok);
  } else {
    diplomat_result_out_value = diplomat::Err(std::monostate());
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ErrorEnum, ResultOpaque> ResultOpaque::new_in_enum_err(int32_t i) {
  auto diplomat_result_raw_out_value = capi::ResultOpaque_new_in_enum_err(i);
  diplomat::result<ErrorEnum, ResultOpaque> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ErrorEnum>(static_cast<ErrorEnum>(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err<ResultOpaque>(ResultOpaque(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value;
}
inline void ResultOpaque::assert_integer(int32_t i) const {
  capi::ResultOpaque_assert_integer(this->inner.get(), i);
}
#endif
