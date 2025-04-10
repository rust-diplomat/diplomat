#ifndef MyString_HPP
#define MyString_HPP

#include "MyString.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    diplomat::capi::MyString* MyString_new(diplomat::capi::DiplomatStringView v);
    
    diplomat::capi::MyString* MyString_new_unsafe(diplomat::capi::DiplomatStringView v);
    
    diplomat::capi::MyString* MyString_new_owned(diplomat::capi::DiplomatStringView v);
    
    diplomat::capi::MyString* MyString_new_from_first(diplomat::capi::DiplomatStringsView v);
    
    void MyString_set_str(diplomat::capi::MyString* self, diplomat::capi::DiplomatStringView new_str);
    
    void MyString_get_str(const diplomat::capi::MyString* self, diplomat::capi::DiplomatWrite* write);
    
    diplomat::capi::DiplomatStringView MyString_get_static_str(void);
    
    void MyString_string_transform(diplomat::capi::DiplomatStringView foo, diplomat::capi::DiplomatWrite* write);
    
    diplomat::capi::DiplomatStringView MyString_borrow(const diplomat::capi::MyString* self);
    
    
    void MyString_destroy(MyString* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<MyString> MyString::new_(std::string_view v) {
  auto result = diplomat::capi::MyString_new({v.data(), v.size()});
  return std::unique_ptr<MyString>(MyString::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<MyString>, diplomat::Utf8Error> MyString::new_unsafe(std::string_view v) {
  if (!diplomat::capi::diplomat_is_str(v.data(), v.size())) {
    return diplomat::Err<diplomat::Utf8Error>();
  }
  auto result = diplomat::capi::MyString_new_unsafe({v.data(), v.size()});
  return diplomat::Ok<std::unique_ptr<MyString>>(std::unique_ptr<MyString>(MyString::FromFFI(result)));
}

inline std::unique_ptr<MyString> MyString::new_owned(std::string_view v) {
  auto result = diplomat::capi::MyString_new_owned({v.data(), v.size()});
  return std::unique_ptr<MyString>(MyString::FromFFI(result));
}

inline std::unique_ptr<MyString> MyString::new_from_first(diplomat::span<const std::string_view> v) {
  auto result = diplomat::capi::MyString_new_from_first({reinterpret_cast<const diplomat::capi::DiplomatStringView*>(v.data()), v.size()});
  return std::unique_ptr<MyString>(MyString::FromFFI(result));
}

inline void MyString::set_str(std::string_view new_str) {
  diplomat::capi::MyString_set_str(this->AsFFI(),
    {new_str.data(), new_str.size()});
}

inline std::string MyString::get_str() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::MyString_get_str(this->AsFFI(),
    &write);
  return output;
}

inline std::string_view MyString::get_static_str() {
  auto result = diplomat::capi::MyString_get_static_str();
  return std::string_view(result.data, result.len);
}

inline diplomat::result<std::string, diplomat::Utf8Error> MyString::string_transform(std::string_view foo) {
  if (!diplomat::capi::diplomat_is_str(foo.data(), foo.size())) {
    return diplomat::Err<diplomat::Utf8Error>();
  }
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::MyString_string_transform({foo.data(), foo.size()},
    &write);
  return diplomat::Ok<std::string>(std::move(output));
}

inline std::string_view MyString::borrow() const {
  auto result = diplomat::capi::MyString_borrow(this->AsFFI());
  return std::string_view(result.data, result.len);
}

inline const diplomat::capi::MyString* MyString::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::MyString*>(this);
}

inline diplomat::capi::MyString* MyString::AsFFI() {
  return reinterpret_cast<diplomat::capi::MyString*>(this);
}

inline const MyString* MyString::FromFFI(const diplomat::capi::MyString* ptr) {
  return reinterpret_cast<const MyString*>(ptr);
}

inline MyString* MyString::FromFFI(diplomat::capi::MyString* ptr) {
  return reinterpret_cast<MyString*>(ptr);
}

inline void MyString::operator delete(void* ptr) {
  diplomat::capi::MyString_destroy(reinterpret_cast<diplomat::capi::MyString*>(ptr));
}


#endif // MyString_HPP
