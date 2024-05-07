#ifndef OptionString_HPP
#define OptionString_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "OptionString.h"

class OptionString;

/**
 * A destruction policy for using OptionString with std::unique_ptr.
 */
struct OptionStringDeleter {
  void operator()(capi::OptionString* l) const noexcept {
    capi::OptionString_destroy(l);
  }
};
class OptionString {
 public:
  static std::optional<OptionString> new_(const std::string_view diplomat_str);
  template<typename W> diplomat::result<std::monostate, std::monostate> write_to_writeable(W& writeable) const;
  diplomat::result<std::string, std::monostate> write() const;

  /**
   * Lifetimes: `this` must live at least as long as the output.
   */
  diplomat::result<const std::string_view, std::monostate> borrow() const;
  inline const capi::OptionString* AsFFI() const { return this->inner.get(); }
  inline capi::OptionString* AsFFIMut() { return this->inner.get(); }
  inline explicit OptionString(capi::OptionString* i) : inner(i) {}
  OptionString() = default;
  OptionString(OptionString&&) noexcept = default;
  OptionString& operator=(OptionString&& other) noexcept = default;
 private:
  std::unique_ptr<capi::OptionString, OptionStringDeleter> inner;
};


inline std::optional<OptionString> OptionString::new_(const std::string_view diplomat_str) {
  auto diplomat_optional_raw_out_value = capi::OptionString_new(diplomat_str.data(), diplomat_str.size());
  std::optional<OptionString> diplomat_optional_out_value;
  if (diplomat_optional_raw_out_value != nullptr) {
    diplomat_optional_out_value = OptionString(diplomat_optional_raw_out_value);
  } else {
    diplomat_optional_out_value = std::nullopt;
  }
  return diplomat_optional_out_value;
}
template<typename W> inline diplomat::result<std::monostate, std::monostate> OptionString::write_to_writeable(W& writeable) const {
  capi::DiplomatWriteable writeable_writer = diplomat::WriteableTrait<W>::Construct(writeable);
  auto diplomat_result_raw_out_value = capi::OptionString_write(this->inner.get(), &writeable_writer);
  diplomat::result<std::monostate, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<std::monostate>(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<std::monostate>(std::monostate());
  }
  return diplomat_result_out_value;
}
inline diplomat::result<std::string, std::monostate> OptionString::write() const {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  auto diplomat_result_raw_out_value = capi::OptionString_write(this->inner.get(), &diplomat_writeable_out);
  diplomat::result<std::monostate, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<std::monostate>(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<std::monostate>(std::monostate());
  }
  return diplomat_result_out_value.replace_ok(std::move(diplomat_writeable_string));
}
inline diplomat::result<const std::string_view, std::monostate> OptionString::borrow() const {
  auto diplomat_result_raw_out_value = capi::OptionString_borrow(this->inner.get());
  diplomat::result<const std::string_view, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
  capi::DiplomatStringView diplomat_str_raw_out_value = diplomat_result_raw_out_value.ok;
  std::string_view str(diplomat_str_raw_out_value.data, diplomat_str_raw_out_value.len);
    diplomat_result_out_value = diplomat::Ok<const std::string_view>(str);
  } else {
    diplomat_result_out_value = diplomat::Err<std::monostate>(std::monostate());
  }
  return diplomat_result_out_value;
}
#endif
