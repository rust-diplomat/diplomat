#ifndef Foo_HPP
#define Foo_HPP

#include "Foo.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "Bar.hpp"
#include "BorrowedFields.hpp"
#include "BorrowedFieldsReturning.hpp"
#include "BorrowedFieldsWithBounds.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    diplomat::capi::Foo* Foo_new(diplomat::capi::DiplomatStringView x);
    
    diplomat::capi::Bar* Foo_get_bar(const diplomat::capi::Foo* self);
    
    diplomat::capi::Foo* Foo_new_static(diplomat::capi::DiplomatStringView x);
    
    diplomat::capi::BorrowedFieldsReturning Foo_as_returning(const diplomat::capi::Foo* self);
    
    diplomat::capi::Foo* Foo_extract_from_fields(diplomat::capi::BorrowedFields fields);
    
    diplomat::capi::Foo* Foo_extract_from_bounds(diplomat::capi::BorrowedFieldsWithBounds bounds, diplomat::capi::DiplomatStringView another_string);
    
    
    void Foo_destroy(Foo* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<Foo> Foo::new_(std::string_view x) {
  auto result = diplomat::capi::Foo_new({x.data(), x.size()});
  return std::unique_ptr<Foo>(Foo::FromFFI(result));
}

inline std::unique_ptr<Bar> Foo::get_bar() const {
  auto result = diplomat::capi::Foo_get_bar(this->AsFFI());
  return std::unique_ptr<Bar>(Bar::FromFFI(result));
}

inline std::unique_ptr<Foo> Foo::new_static(std::string_view x) {
  auto result = diplomat::capi::Foo_new_static({x.data(), x.size()});
  return std::unique_ptr<Foo>(Foo::FromFFI(result));
}

inline BorrowedFieldsReturning Foo::as_returning() const {
  auto result = diplomat::capi::Foo_as_returning(this->AsFFI());
  return BorrowedFieldsReturning::FromFFI(result);
}

inline std::unique_ptr<Foo> Foo::extract_from_fields(BorrowedFields fields) {
  auto result = diplomat::capi::Foo_extract_from_fields(fields.AsFFI());
  return std::unique_ptr<Foo>(Foo::FromFFI(result));
}

inline std::unique_ptr<Foo> Foo::extract_from_bounds(BorrowedFieldsWithBounds bounds, std::string_view another_string) {
  auto result = diplomat::capi::Foo_extract_from_bounds(bounds.AsFFI(),
    {another_string.data(), another_string.size()});
  return std::unique_ptr<Foo>(Foo::FromFFI(result));
}

inline const diplomat::capi::Foo* Foo::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Foo*>(this);
}

inline diplomat::capi::Foo* Foo::AsFFI() {
  return reinterpret_cast<diplomat::capi::Foo*>(this);
}

inline const Foo* Foo::FromFFI(const diplomat::capi::Foo* ptr) {
  return reinterpret_cast<const Foo*>(ptr);
}

inline Foo* Foo::FromFFI(diplomat::capi::Foo* ptr) {
  return reinterpret_cast<Foo*>(ptr);
}

inline void Foo::operator delete(void* ptr) {
  diplomat::capi::Foo_destroy(reinterpret_cast<diplomat::capi::Foo*>(ptr));
}


#endif // Foo_HPP
