#ifndef MyString_HPP
#define MyString_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "MyString.h"

class MyString;

/**
 * A destruction policy for using MyString with std::unique_ptr.
 */
struct MyStringDeleter {
  void operator()(capi::MyString* l) const noexcept {
    capi::MyString_destroy(l);
  }
};
class MyString {
 public:
  static MyString new_(const std::string_view v);

  /**
   * Warning: Passing ill-formed UTF-8 is undefined behavior (and may be memory-unsafe).
   */
  static MyString new_unsafe(const std::string_view v);
  void set_str(const std::string_view new_str);
  template<typename W> void get_str_to_writeable(W& writeable) const;
  std::string get_str() const;
  inline const capi::MyString* AsFFI() const { return this->inner.get(); }
  inline capi::MyString* AsFFIMut() { return this->inner.get(); }
  inline explicit MyString(capi::MyString* i) : inner(i) {}
  MyString() = default;
  MyString(MyString&&) noexcept = default;
  MyString& operator=(MyString&& other) noexcept = default;
 private:
  std::unique_ptr<capi::MyString, MyStringDeleter> inner;
};


inline MyString MyString::new_(const std::string_view v) {
  return MyString(capi::MyString_new(v.data(), v.size()));
}
inline MyString MyString::new_unsafe(const std::string_view v) {
  return MyString(capi::MyString_new_unsafe(v.data(), v.size()));
}
inline void MyString::set_str(const std::string_view new_str) {
  capi::MyString_set_str(this->inner.get(), new_str.data(), new_str.size());
}
template<typename W> inline void MyString::get_str_to_writeable(W& writeable) const {
  capi::DiplomatWriteable writeable_writer = diplomat::WriteableTrait<W>::Construct(writeable);
  capi::MyString_get_str(this->inner.get(), &writeable_writer);
}
inline std::string MyString::get_str() const {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  capi::MyString_get_str(this->inner.get(), &diplomat_writeable_out);
  return diplomat_writeable_string;
}
#endif
