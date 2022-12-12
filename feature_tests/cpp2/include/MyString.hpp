#ifndef MyString_HPP
#define MyString_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "MyString.h"

#include "MyString.d.hpp"


inline std::unique_ptr<MyString> MyString::new_(std::string_view v) {
  auto result = capi::MyString_new(v.data(),
    v.size());
  return std::unique_ptr(MyString::FromFFI(result));
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

inline MyString::~MyString() {
  capi::MyString_destroy(AsFFI());
}


#endif // MyString_HPP
