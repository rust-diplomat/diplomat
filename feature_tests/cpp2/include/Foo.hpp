#ifndef Foo_HPP
#define Foo_HPP

#include "Foo.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "Bar.hpp"
#include "BorrowedFields.hpp"
#include "BorrowedFieldsReturning.hpp"
#include "BorrowedFieldsWithBounds.hpp"


namespace capi {
    extern "C" {
    
    Foo* Foo_new(const char* x_data, size_t x_len);
    
    Bar* Foo_get_bar(const Foo* self);
    
    Foo* Foo_new_static(const char* x_data, size_t x_len);
    
    BorrowedFieldsReturning Foo_as_returning(const Foo* self);
    
    Foo* Foo_extract_from_fields(BorrowedFields fields);
    
    Foo* Foo_extract_from_bounds(BorrowedFieldsWithBounds bounds, const char* another_string_data, size_t another_string_len);
    
    
    void Foo_destroy(Foo* self);
    
    } // extern "C"
}

inline std::unique_ptr<Foo> Foo::new_(std::string_view x) {
  auto result = capi::Foo_new(x.data(),
    x.size());
  return std::unique_ptr<Foo>(Foo::FromFFI(result));
}

inline std::unique_ptr<Bar> Foo::get_bar() const {
  auto result = capi::Foo_get_bar(this->AsFFI());
  return std::unique_ptr<Bar>(Bar::FromFFI(result));
}

inline std::unique_ptr<Foo> Foo::new_static(std::string_view x) {
  auto result = capi::Foo_new_static(x.data(),
    x.size());
  return std::unique_ptr<Foo>(Foo::FromFFI(result));
}

inline BorrowedFieldsReturning Foo::as_returning() const {
  auto result = capi::Foo_as_returning(this->AsFFI());
  return BorrowedFieldsReturning::FromFFI(result);
}

inline std::unique_ptr<Foo> Foo::extract_from_fields(BorrowedFields fields) {
  auto result = capi::Foo_extract_from_fields(fields.AsFFI());
  return std::unique_ptr<Foo>(Foo::FromFFI(result));
}

inline std::unique_ptr<Foo> Foo::extract_from_bounds(BorrowedFieldsWithBounds bounds, std::string_view another_string) {
  auto result = capi::Foo_extract_from_bounds(bounds.AsFFI(),
    another_string.data(),
    another_string.size());
  return std::unique_ptr<Foo>(Foo::FromFFI(result));
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

inline void Foo::operator delete(void* ptr) {
  capi::Foo_destroy(reinterpret_cast<capi::Foo*>(ptr));
}


#endif // Foo_HPP
