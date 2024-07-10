#ifndef MyString_HPP
#define MyString_HPP

#include "MyString.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    extern "C" {
    
    ::capi::MyString* MyString_new(const char* v_data, size_t v_len);
    
    ::capi::MyString* MyString_new_unsafe(const char* v_data, size_t v_len);
    
    ::capi::MyString* MyString_new_owned(const char* v_data, size_t v_len);
    
    void MyString_set_str(::capi::MyString* self, const char* new_str_data, size_t new_str_len);
    
    void MyString_get_str(const ::capi::MyString* self, diplomat::capi::DiplomatWrite* write);
    
    DiplomatStringView MyString_get_boxed_str(const ::capi::MyString* self);
    
    
    void MyString_destroy(MyString* self);
    
    } // extern "C"
}
inline std::unique_ptr<MyString> MyString::new_(std::string_view v) {
  auto result = capi::MyString_new(v.data(),
    v.size());
  return std::unique_ptr<MyString>(MyString::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<MyString>, diplomat::Utf8Error> MyString::new_unsafe(std::string_view v) {
  if (!diplomat::capi::diplomat_is_str(v.data(), v.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  auto result = capi::MyString_new_unsafe(v.data(),
    v.size());
  return diplomat::Ok<std::unique_ptr<MyString>>(std::move(std::unique_ptr<MyString>(MyString::FromFFI(result))));
}

inline std::unique_ptr<MyString> MyString::new_owned(std::string_view v) {
  auto result = capi::MyString_new_owned(v.data(),
    v.size());
  return std::unique_ptr<MyString>(MyString::FromFFI(result));
}

inline void MyString::set_str(std::string_view new_str) {
  capi::MyString_set_str(this->AsFFI(),
    new_str.data(),
    new_str.size());
}

inline std::string MyString::get_str() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::MyString_get_str(this->AsFFI(),
    &write);
  return output;
}

inline std::string_view MyString::get_boxed_str() const {
  auto result = capi::MyString_get_boxed_str(this->AsFFI());
  return std::string_view(result.data, result.len);
}

inline const ::capi::MyString* MyString::AsFFI() const {
  return reinterpret_cast<const ::capi::MyString*>(this);
}

inline ::capi::MyString* MyString::AsFFI() {
  return reinterpret_cast<::capi::MyString*>(this);
}

inline const MyString* MyString::FromFFI(const ::capi::MyString* ptr) {
  return reinterpret_cast<const MyString*>(ptr);
}

inline MyString* MyString::FromFFI(::capi::MyString* ptr) {
  return reinterpret_cast<MyString*>(ptr);
}

inline void MyString::operator delete(void* ptr) {
  capi::MyString_destroy(reinterpret_cast<::capi::MyString*>(ptr));
}


#endif // MyString_HPP
