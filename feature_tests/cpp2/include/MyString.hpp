#ifndef MyString_HPP
#define MyString_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.h"
#include "MyString.d.hpp"
#include "MyString.h"





inline std::unique_ptr<MyString> MyString::new_(std::string_view v) {
  capi::MyString_new(v.data(),
    v.size());
  // TODO
}

inline void MyString::set_str(std::string_view new_str) {
  capi::MyString_set_str(this->AsFFI(),
    new_str.data(),
    new_str.size());
  // TODO
}

inline std::string MyString::get_str() const {
  capi::MyString_get_str(this->AsFFI());
  // TODO
}

inline const capi::MyString* MyString::AsFFI() const {
  return reinterpret_cast<const capi::MyString*>(this);
}
inline capi::MyString* MyString::AsFFI() {
  return reinterpret_cast<capi::MyString*>(this);
}
inline MyString::~MyString() {
  capi::MyString_destroy(AsFFI());
}


#endif // MyString_HPP
