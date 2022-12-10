#ifndef Foo_HPP
#define Foo_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "Bar.d.hpp"
#include "Foo.d.hpp"
#include "Foo.h"





inline std::unique_ptr<Foo> Foo::new_(std::string_view x) {
  auto result = capi::Foo_new(x.data(),
    x.size());
  return std::unique_ptr(Foo::FromFFI(result));
}

inline std::unique_ptr<Bar> Foo::get_bar() const {
  auto result = capi::Foo_get_bar(this->AsFFI());
  return std::unique_ptr(Foo::FromFFI(result));
}

inline std::unique_ptr<Foo> Foo::new_static(std::string_view x) {
  auto result = capi::Foo_new_static(x.data(),
    x.size());
  return std::unique_ptr(Foo::FromFFI(result));
}

inline BorrowedFieldsReturning Foo::as_returning() const {
  auto result = capi::Foo_as_returning(this->AsFFI());
  return Foo::FromFFI(result);
}

inline std::unique_ptr<Foo> Foo::extract_from_fields(BorrowedFields fields) {
  auto result = capi::Foo_extract_from_fields(fields.AsFFI());
  return std::unique_ptr(Foo::FromFFI(result));
}

inline const capi::Foo* Foo::AsFFI() const {
  return reinterpret_cast<const capi::Foo*>(this);
}
inline capi::Foo* Foo::AsFFI() {
  return reinterpret_cast<capi::Foo*>(this);
}
inline const Foo* Foo::FromFFI(const capi::Foo* ptr) {
  return reinterpret_cast<const Foo*>(ptr);
}
inline Foo* Foo::FromFFI(capi::Foo* ptr) {
  return reinterpret_cast<Foo*>(ptr);
}
inline Foo::~Foo() {
  capi::Foo_destroy(AsFFI());
}


#endif // Foo_HPP
