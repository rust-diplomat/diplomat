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
#include "MyString.h"


inline std::unique_ptr<MyString> MyString::new_(std::string_view v) {
  auto result = capi::MyString_new(v.data(),
    v.size());
  return std::unique_ptr<MyString>(MyString::FromFFI(result));
}

inline std::unique_ptr<MyString> MyString::new_unsafe(std::string_view v) {
  auto result = capi::MyString_new_unsafe(v.data(),
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
  capi::DiplomatWriteable writeable = diplomat::WriteableFromString(output);
  capi::MyString_get_str(this->AsFFI(),
    &writeable);
  return output;
}

inline const capi::MyString* MyString::AsFFI() const {
  return reinterpret_cast<const capi::MyString*>(this);
}

inline capi::MyString* MyString::AsFFI() {
  return reinterpret_cast<capi::MyString*>(this);
}

inline const MyString* MyString::FromFFI(const capi::MyString* ptr) {
  return reinterpret_cast<const MyString*>(ptr);
}

inline MyString* MyString::FromFFI(capi::MyString* ptr) {
  return reinterpret_cast<MyString*>(ptr);
}

inline void MyString::operator delete(void* ptr) {
  capi::MyString_destroy(reinterpret_cast<capi::MyString*>(ptr));
}


#endif // MyString_HPP
