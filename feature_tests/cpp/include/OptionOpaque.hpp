#ifndef OptionOpaque_HPP
#define OptionOpaque_HPP
#include "diplomat_runtime.hpp"
#include <algorithm>
#include <memory>
#include <optional>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <variant>

#include "OptionOpaque.h"

class OptionOpaque;
struct OptionStruct;

/**
 * A destruction policy for using OptionOpaque with std::unique_ptr.
 */
struct OptionOpaqueDeleter {
  void operator()(capi::OptionOpaque *l) const noexcept {
    capi::OptionOpaque_destroy(l);
  }
};
class OptionOpaque {
public:
  static std::optional<OptionOpaque> new_(int32_t i);
  static std::optional<OptionOpaque> new_none();
  static diplomat::result<OptionStruct, std::monostate> returns();
  diplomat::result<intptr_t, std::monostate> option_isize() const;
  diplomat::result<size_t, std::monostate> option_usize() const;
  diplomat::result<int32_t, std::monostate> option_i32() const;
  diplomat::result<uint32_t, std::monostate> option_u32() const;
  static OptionStruct new_struct();
  static OptionStruct new_struct_nones();
  void assert_integer(int32_t i) const;
  static bool option_opaque_argument(const OptionOpaque *arg);
  inline const capi::OptionOpaque *AsFFI() const { return this->inner.get(); }
  inline capi::OptionOpaque *AsFFIMut() { return this->inner.get(); }
  inline explicit OptionOpaque(capi::OptionOpaque *i) : inner(i) {}
  OptionOpaque() = default;
  OptionOpaque(OptionOpaque &&) noexcept = default;
  OptionOpaque &operator=(OptionOpaque &&other) noexcept = default;

private:
  std::unique_ptr<capi::OptionOpaque, OptionOpaqueDeleter> inner;
};

#include "OptionStruct.hpp"

inline std::optional<OptionOpaque> OptionOpaque::new_(int32_t i) {
  auto diplomat_optional_raw_out_value = capi::OptionOpaque_new(i);
  std::optional<OptionOpaque> diplomat_optional_out_value;
  if (diplomat_optional_raw_out_value != nullptr) {
    diplomat_optional_out_value = OptionOpaque(diplomat_optional_raw_out_value);
  } else {
    diplomat_optional_out_value = std::nullopt;
  }
  return diplomat_optional_out_value;
}
inline std::optional<OptionOpaque> OptionOpaque::new_none() {
  auto diplomat_optional_raw_out_value = capi::OptionOpaque_new_none();
  std::optional<OptionOpaque> diplomat_optional_out_value;
  if (diplomat_optional_raw_out_value != nullptr) {
    diplomat_optional_out_value = OptionOpaque(diplomat_optional_raw_out_value);
  } else {
    diplomat_optional_out_value = std::nullopt;
  }
  return diplomat_optional_out_value;
}
inline std::optional<OptionStruct> OptionOpaque::returns() {
  auto diplomat_result_raw_out_value = capi::OptionOpaque_returns();
  std::optional<OptionStruct> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    capi::OptionStruct diplomat_raw_struct_out_value =
        diplomat_result_raw_out_value.ok;
    auto diplomat_optional_raw_out_value_a = diplomat_raw_struct_out_value.a;
    std::optional<OptionOpaque> diplomat_optional_out_value_a;
    if (diplomat_optional_raw_out_value_a != nullptr) {
      diplomat_optional_out_value_a =
          OptionOpaque(diplomat_optional_raw_out_value_a);
    } else {
      diplomat_optional_out_value_a = std::nullopt;
    }
    auto diplomat_optional_raw_out_value_b = diplomat_raw_struct_out_value.b;
    std::optional<OptionOpaqueChar> diplomat_optional_out_value_b;
    if (diplomat_optional_raw_out_value_b != nullptr) {
      diplomat_optional_out_value_b =
          OptionOpaqueChar(diplomat_optional_raw_out_value_b);
    } else {
      diplomat_optional_out_value_b = std::nullopt;
    }
    auto diplomat_optional_raw_out_value_d = diplomat_raw_struct_out_value.d;
    std::optional<OptionOpaque> diplomat_optional_out_value_d;
    if (diplomat_optional_raw_out_value_d != nullptr) {
      diplomat_optional_out_value_d =
          OptionOpaque(diplomat_optional_raw_out_value_d);
    } else {
      diplomat_optional_out_value_d = std::nullopt;
    }
    diplomat_result_out_value = std::optional<OptionStruct>(
        OptionStruct{.a = std::move(diplomat_optional_out_value_a),
                     .b = std::move(diplomat_optional_out_value_b),
                     .c = std::move(diplomat_raw_struct_out_value.c),
                     .d = std::move(diplomat_optional_out_value_d)});
  } else {
    diplomat_result_out_value = std::nullopt;
  }
  return diplomat_result_out_value;
}
inline diplomat::result<intptr_t, std::monostate>
OptionOpaque::option_isize() const {
  auto diplomat_result_raw_out_value =
      capi::OptionOpaque_option_isize(this->inner.get());
  diplomat::result<intptr_t, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value =
        diplomat::Ok<intptr_t>(diplomat_result_raw_out_value.ok);
  } else {
    diplomat_result_out_value = diplomat::Err<std::monostate>(std::monostate());
  }
  return diplomat_result_out_value;
}
inline diplomat::result<size_t, std::monostate>
OptionOpaque::option_usize() const {
  auto diplomat_result_raw_out_value =
      capi::OptionOpaque_option_usize(this->inner.get());
  diplomat::result<size_t, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value =
        diplomat::Ok<size_t>(diplomat_result_raw_out_value.ok);
  } else {
    diplomat_result_out_value = diplomat::Err<std::monostate>(std::monostate());
  }
  return diplomat_result_out_value;
}
inline diplomat::result<int32_t, std::monostate>
OptionOpaque::option_i32() const {
  auto diplomat_result_raw_out_value =
      capi::OptionOpaque_option_i32(this->inner.get());
  diplomat::result<int32_t, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value =
        diplomat::Ok<int32_t>(diplomat_result_raw_out_value.ok);
  } else {
    diplomat_result_out_value = diplomat::Err<std::monostate>(std::monostate());
  }
  return diplomat_result_out_value;
}
inline diplomat::result<uint32_t, std::monostate>
OptionOpaque::option_u32() const {
  auto diplomat_result_raw_out_value =
      capi::OptionOpaque_option_u32(this->inner.get());
  diplomat::result<uint32_t, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value =
        diplomat::Ok<uint32_t>(diplomat_result_raw_out_value.ok);
  } else {
    diplomat_result_out_value = diplomat::Err<std::monostate>(std::monostate());
  }
  return diplomat_result_out_value;
}
inline OptionStruct OptionOpaque::new_struct() {
  capi::OptionStruct diplomat_raw_struct_out_value =
      capi::OptionOpaque_new_struct();
  auto diplomat_optional_raw_out_value_a = diplomat_raw_struct_out_value.a;
  std::optional<OptionOpaque> diplomat_optional_out_value_a;
  if (diplomat_optional_raw_out_value_a != nullptr) {
    diplomat_optional_out_value_a =
        OptionOpaque(diplomat_optional_raw_out_value_a);
  } else {
    diplomat_optional_out_value_a = std::nullopt;
  }
  auto diplomat_optional_raw_out_value_b = diplomat_raw_struct_out_value.b;
  std::optional<OptionOpaqueChar> diplomat_optional_out_value_b;
  if (diplomat_optional_raw_out_value_b != nullptr) {
    diplomat_optional_out_value_b =
        OptionOpaqueChar(diplomat_optional_raw_out_value_b);
  } else {
    diplomat_optional_out_value_b = std::nullopt;
  }
  auto diplomat_optional_raw_out_value_d = diplomat_raw_struct_out_value.d;
  std::optional<OptionOpaque> diplomat_optional_out_value_d;
  if (diplomat_optional_raw_out_value_d != nullptr) {
    diplomat_optional_out_value_d =
        OptionOpaque(diplomat_optional_raw_out_value_d);
  } else {
    diplomat_optional_out_value_d = std::nullopt;
  }
  return OptionStruct{.a = std::move(diplomat_optional_out_value_a),
                      .b = std::move(diplomat_optional_out_value_b),
                      .c = std::move(diplomat_raw_struct_out_value.c),
                      .d = std::move(diplomat_optional_out_value_d)};
}
inline OptionStruct OptionOpaque::new_struct_nones() {
  capi::OptionStruct diplomat_raw_struct_out_value =
      capi::OptionOpaque_new_struct_nones();
  auto diplomat_optional_raw_out_value_a = diplomat_raw_struct_out_value.a;
  std::optional<OptionOpaque> diplomat_optional_out_value_a;
  if (diplomat_optional_raw_out_value_a != nullptr) {
    diplomat_optional_out_value_a =
        OptionOpaque(diplomat_optional_raw_out_value_a);
  } else {
    diplomat_optional_out_value_a = std::nullopt;
  }
  auto diplomat_optional_raw_out_value_b = diplomat_raw_struct_out_value.b;
  std::optional<OptionOpaqueChar> diplomat_optional_out_value_b;
  if (diplomat_optional_raw_out_value_b != nullptr) {
    diplomat_optional_out_value_b =
        OptionOpaqueChar(diplomat_optional_raw_out_value_b);
  } else {
    diplomat_optional_out_value_b = std::nullopt;
  }
  auto diplomat_optional_raw_out_value_d = diplomat_raw_struct_out_value.d;
  std::optional<OptionOpaque> diplomat_optional_out_value_d;
  if (diplomat_optional_raw_out_value_d != nullptr) {
    diplomat_optional_out_value_d =
        OptionOpaque(diplomat_optional_raw_out_value_d);
  } else {
    diplomat_optional_out_value_d = std::nullopt;
  }
  return OptionStruct{.a = std::move(diplomat_optional_out_value_a),
                      .b = std::move(diplomat_optional_out_value_b),
                      .c = std::move(diplomat_raw_struct_out_value.c),
                      .d = std::move(diplomat_optional_out_value_d)};
}
inline void OptionOpaque::assert_integer(int32_t i) const {
  capi::OptionOpaque_assert_integer(this->inner.get(), i);
}
inline bool OptionOpaque::option_opaque_argument(const OptionOpaque *arg) {
  return capi::OptionOpaque_option_opaque_argument((arg) ? arg->AsFFI()
                                                         : nullptr);
}
#endif
