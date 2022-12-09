#ifndef Foo_HPP
#define Foo_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.h"
#include "Foo.d.hpp"
#include "Foo.h"





inline std::unique_ptr<Foo> Foo::new_(std::string_view x) {
  capi::Foo_new(x.data(),
    x.size());
  // TODO
}

inline std::unique_ptr<Bar> Foo::get_bar() const {
  capi::Foo_get_bar(this->AsFFI());
  // TODO
}

inline std::unique_ptr<Foo> Foo::new_static(std::string_view x) {
  capi::Foo_new_static(x.data(),
    x.size());
  // TODO
}

inline BorrowedFieldsReturning Foo::as_returning() const {
  capi::Foo_as_returning(this->AsFFI());
  // TODO
}

inline std::unique_ptr<Foo> Foo::extract_from_fields(BorrowedFields fields) {
  capi::Foo_extract_from_fields(fields.AsFFI());
  // TODO
}

inline const capi::Foo* Foo::AsFFI() const {
  return reinterpret_cast<const capi::Foo*>(this);
}
inline capi::Foo* Foo::AsFFI() {
  return reinterpret_cast<capi::Foo*>(this);
}
inline Foo::~Foo() {
  capi::Foo_destroy(AsFFI());
}


#endif // Foo_HPP
