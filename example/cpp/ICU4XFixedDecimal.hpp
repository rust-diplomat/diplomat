#ifndef ICU4XFixedDecimal_HPP
#define ICU4XFixedDecimal_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include <variant>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XFixedDecimal.h"
}

class ICU4XFixedDecimal;

struct ICU4XFixedDecimalDeleter {
  void operator()(capi::ICU4XFixedDecimal* l) const noexcept {
    capi::ICU4XFixedDecimal_destroy(l);
  }
};
class ICU4XFixedDecimal {
 public:
  static ICU4XFixedDecimal new_(int32_t v);
  void multiply_pow10(int16_t power);
  void negate();
  template<typename W> diplomat::result<std::monostate, std::monostate> to_string_to_writeable(W& to);
  diplomat::result<std::string, std::monostate> to_string();
  inline const capi::ICU4XFixedDecimal* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XFixedDecimal* AsFFIMut() { return this->inner.get(); }
  ICU4XFixedDecimal(capi::ICU4XFixedDecimal* i) : inner(i) {}
 private:
  std::unique_ptr<capi::ICU4XFixedDecimal, ICU4XFixedDecimalDeleter> inner;
};


ICU4XFixedDecimal ICU4XFixedDecimal::new_(int32_t v) {
  return ICU4XFixedDecimal(capi::ICU4XFixedDecimal_new(v));
}
void ICU4XFixedDecimal::multiply_pow10(int16_t power) {
  capi::ICU4XFixedDecimal_multiply_pow10(this->inner.get(), power);
}
void ICU4XFixedDecimal::negate() {
  capi::ICU4XFixedDecimal_negate(this->inner.get());
}
template<typename W> diplomat::result<std::monostate, std::monostate> ICU4XFixedDecimal::to_string_to_writeable(W& to) {
  capi::DiplomatWriteable to_writer = diplomat::WriteableTrait<W>::Construct(to);
  auto diplomat_result_raw_out_value = capi::ICU4XFixedDecimal_to_string(this->inner.get(), &to_writer);
  diplomat::result<std::monostate, std::monostate> diplomat_result_out_value;
  diplomat_result_out_value.is_ok = diplomat_result_raw_out_value.is_ok;
  if (diplomat_result_raw_out_value.is_ok) {
  } else {
  }
  return diplomat_result_out_value;
}
diplomat::result<std::string, std::monostate> ICU4XFixedDecimal::to_string() {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  auto diplomat_result_raw_out_value = capi::ICU4XFixedDecimal_to_string(this->inner.get(), &diplomat_writeable_out);
  diplomat::result<std::monostate, std::monostate> diplomat_result_out_value;
  diplomat_result_out_value.is_ok = diplomat_result_raw_out_value.is_ok;
  if (diplomat_result_raw_out_value.is_ok) {
  } else {
  }
  diplomat::result<std::monostate, std::monostate> out_value = diplomat_result_out_value;
  if (out_value.is_ok) {
    return diplomat::result<std::string, std::monostate>::new_ok(diplomat_writeable_string);
  } else {
    return diplomat::result<std::string, std::monostate>::new_err_void();
  }
}
#endif
