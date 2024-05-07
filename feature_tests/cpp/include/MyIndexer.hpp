#ifndef MyIndexer_HPP
#define MyIndexer_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "MyIndexer.h"


/**
 * A destruction policy for using MyIndexer with std::unique_ptr.
 */
struct MyIndexerDeleter {
  void operator()(capi::MyIndexer* l) const noexcept {
    capi::namespace_MyIndexer_destroy(l);
  }
};
class MyIndexer {
 public:

  /**
   * Lifetimes: `this` must live at least as long as the output.
   */
  diplomat::result<const std::string_view, std::monostate> get(size_t i) const;
  inline const capi::MyIndexer* AsFFI() const { return this->inner.get(); }
  inline capi::MyIndexer* AsFFIMut() { return this->inner.get(); }
  inline explicit MyIndexer(capi::MyIndexer* i) : inner(i) {}
  MyIndexer() = default;
  MyIndexer(MyIndexer&&) noexcept = default;
  MyIndexer& operator=(MyIndexer&& other) noexcept = default;
 private:
  std::unique_ptr<capi::MyIndexer, MyIndexerDeleter> inner;
};


inline diplomat::result<const std::string_view, std::monostate> MyIndexer::get(size_t i) const {
  auto diplomat_result_raw_out_value = capi::namespace_MyIndexer_get(this->inner.get(), i);
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
