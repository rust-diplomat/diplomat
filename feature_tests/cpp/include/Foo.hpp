#ifndef SOMELIB_Foo_HPP
#define SOMELIB_Foo_HPP

#include "Foo.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "Bar.hpp"
#include "BorrowedFields.hpp"
#include "BorrowedFieldsReturning.hpp"
#include "BorrowedFieldsWithBounds.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    somelib::capi::Foo* Foo_new(somelib::diplomat::capi::DiplomatStringView x);

    somelib::capi::Bar* Foo_get_bar(const somelib::capi::Foo* self);

    somelib::capi::Foo* Foo_new_static(somelib::diplomat::capi::DiplomatStringView x);

    somelib::capi::BorrowedFieldsReturning Foo_as_returning(const somelib::capi::Foo* self);

    somelib::capi::Foo* Foo_extract_from_fields(somelib::capi::BorrowedFields fields);

    somelib::capi::Foo* Foo_extract_from_bounds(somelib::capi::BorrowedFieldsWithBounds bounds, somelib::diplomat::capi::DiplomatStringView another_string);

    void Foo_destroy(Foo* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<somelib::Foo> somelib::Foo::new_(std::string_view x) {
    auto result = somelib::capi::Foo_new({x.data(), x.size()});
    return std::unique_ptr<somelib::Foo>(somelib::Foo::FromFFI(result));
}

inline std::unique_ptr<somelib::Bar> somelib::Foo::get_bar() const {
    auto result = somelib::capi::Foo_get_bar(this->AsFFI());
    return std::unique_ptr<somelib::Bar>(somelib::Bar::FromFFI(result));
}

inline std::unique_ptr<somelib::Foo> somelib::Foo::new_static(std::string_view x) {
    auto result = somelib::capi::Foo_new_static({x.data(), x.size()});
    return std::unique_ptr<somelib::Foo>(somelib::Foo::FromFFI(result));
}

inline somelib::BorrowedFieldsReturning somelib::Foo::as_returning() const {
    auto result = somelib::capi::Foo_as_returning(this->AsFFI());
    return somelib::BorrowedFieldsReturning::FromFFI(result);
}

inline std::unique_ptr<somelib::Foo> somelib::Foo::extract_from_fields(somelib::BorrowedFields fields) {
    auto result = somelib::capi::Foo_extract_from_fields(fields.AsFFI());
    return std::unique_ptr<somelib::Foo>(somelib::Foo::FromFFI(result));
}

inline std::unique_ptr<somelib::Foo> somelib::Foo::extract_from_bounds(somelib::BorrowedFieldsWithBounds bounds, std::string_view another_string) {
    auto result = somelib::capi::Foo_extract_from_bounds(bounds.AsFFI(),
        {another_string.data(), another_string.size()});
    return std::unique_ptr<somelib::Foo>(somelib::Foo::FromFFI(result));
}

inline const somelib::capi::Foo* somelib::Foo::AsFFI() const {
    return reinterpret_cast<const somelib::capi::Foo*>(this);
}

inline somelib::capi::Foo* somelib::Foo::AsFFI() {
    return reinterpret_cast<somelib::capi::Foo*>(this);
}

inline const somelib::Foo* somelib::Foo::FromFFI(const somelib::capi::Foo* ptr) {
    return reinterpret_cast<const somelib::Foo*>(ptr);
}

inline somelib::Foo* somelib::Foo::FromFFI(somelib::capi::Foo* ptr) {
    return reinterpret_cast<somelib::Foo*>(ptr);
}

inline void somelib::Foo::operator delete(void* ptr) {
    somelib::capi::Foo_destroy(reinterpret_cast<somelib::capi::Foo*>(ptr));
}


#endif // SOMELIB_Foo_HPP
