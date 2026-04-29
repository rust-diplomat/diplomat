#ifndef SOMELIB_MyString_HPP
#define SOMELIB_MyString_HPP

#include "MyString.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    somelib::capi::MyString* MyString_new(somelib::diplomat::capi::DiplomatStringView v);

    somelib::capi::MyString* MyString_new_unsafe(somelib::diplomat::capi::DiplomatStringView v);

    somelib::capi::MyString* MyString_new_from_first(somelib::diplomat::capi::DiplomatStringsView v);

    somelib::capi::MyString* MyString_new_from_utf16(somelib::diplomat::capi::DiplomatStrings16View v);

    void MyString_set_str(somelib::capi::MyString* self, somelib::diplomat::capi::DiplomatStringView new_str);

    void MyString_get_str(const somelib::capi::MyString* self, somelib::diplomat::capi::DiplomatWrite* write);

    somelib::diplomat::capi::DiplomatStringView MyString_get_static_str(void);

    void MyString_string_transform(somelib::diplomat::capi::DiplomatStringView foo, somelib::diplomat::capi::DiplomatWrite* write);

    somelib::diplomat::capi::DiplomatStringView MyString_borrow(const somelib::capi::MyString* self);

    void MyString_slice_of_opaques(somelib::capi::DiplomatMyStringView sl, somelib::diplomat::capi::DiplomatWrite* write);

    void MyString_optional_slice_of_opaques(somelib::capi::DiplomatMyStringView sl, somelib::diplomat::capi::DiplomatWrite* write);

    void MyString_destroy(MyString* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<somelib::MyString> somelib::MyString::new_(std::string_view v) {
    auto result = somelib::capi::MyString_new({v.data(), v.size()});
    return std::unique_ptr<somelib::MyString>(somelib::MyString::FromFFI(result));
}

inline somelib::diplomat::result<std::unique_ptr<somelib::MyString>, somelib::diplomat::Utf8Error> somelib::MyString::new_unsafe(std::string_view v) {
    if (!somelib::diplomat::capi::diplomat_is_str(v.data(), v.size())) {
    return somelib::diplomat::Err<somelib::diplomat::Utf8Error>();
  }
    auto result = somelib::capi::MyString_new_unsafe({v.data(), v.size()});
    return somelib::diplomat::Ok<std::unique_ptr<somelib::MyString>>(std::unique_ptr<somelib::MyString>(somelib::MyString::FromFFI(result)));
}

inline std::unique_ptr<somelib::MyString> somelib::MyString::new_from_first(somelib::diplomat::span<const diplomat::string_view_for_slice> v) {
    auto result = somelib::capi::MyString_new_from_first({reinterpret_cast<const somelib::diplomat::capi::DiplomatStringView*>(v.data()), v.size()});
    return std::unique_ptr<somelib::MyString>(somelib::MyString::FromFFI(result));
}

inline std::unique_ptr<somelib::MyString> somelib::MyString::new_from_utf16(somelib::diplomat::span<const diplomat::u16string_view_for_slice> v) {
    auto result = somelib::capi::MyString_new_from_utf16({reinterpret_cast<const somelib::diplomat::capi::DiplomatString16View*>(v.data()), v.size()});
    return std::unique_ptr<somelib::MyString>(somelib::MyString::FromFFI(result));
}

inline void somelib::MyString::set_str(std::string_view new_str) {
    somelib::capi::MyString_set_str(this->AsFFI(),
        {new_str.data(), new_str.size()});
}

inline std::string somelib::MyString::get_str() const {
    std::string output;
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteFromString(output);
    somelib::capi::MyString_get_str(this->AsFFI(),
        &write);
    return output;
}
template<typename W>
inline void somelib::MyString::get_str_write(W& writeable) const {
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteTrait<W>::Construct(writeable);
    somelib::capi::MyString_get_str(this->AsFFI(),
        &write);
}

inline std::string_view somelib::MyString::get_static_str() {
    auto result = somelib::capi::MyString_get_static_str();
    return std::string_view(result.data, result.len);
}

inline somelib::diplomat::result<std::string, somelib::diplomat::Utf8Error> somelib::MyString::string_transform(std::string_view foo) {
    if (!somelib::diplomat::capi::diplomat_is_str(foo.data(), foo.size())) {
    return somelib::diplomat::Err<somelib::diplomat::Utf8Error>();
  }
    std::string output;
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteFromString(output);
    somelib::capi::MyString_string_transform({foo.data(), foo.size()},
        &write);
    return somelib::diplomat::Ok<std::string>(std::move(output));
}
template<typename W>
inline somelib::diplomat::result<std::monostate, somelib::diplomat::Utf8Error> somelib::MyString::string_transform_write(std::string_view foo, W& writeable) {
    if (!somelib::diplomat::capi::diplomat_is_str(foo.data(), foo.size())) {
    return somelib::diplomat::Err<somelib::diplomat::Utf8Error>();
  }
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteTrait<W>::Construct(writeable);
    somelib::capi::MyString_string_transform({foo.data(), foo.size()},
        &write);
    return somelib::diplomat::Ok<std::monostate>();
}

inline std::string_view somelib::MyString::borrow() const {
    auto result = somelib::capi::MyString_borrow(this->AsFFI());
    return std::string_view(result.data, result.len);
}

inline std::string somelib::MyString::slice_of_opaques(somelib::diplomat::span<const somelib::MyString*> sl) {
    std::string output;
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteFromString(output);
    somelib::capi::MyString_slice_of_opaques({reinterpret_cast<const somelib::capi::MyString**>(sl.data()), sl.size()},
        &write);
    return output;
}
template<typename W>
inline void somelib::MyString::slice_of_opaques_write(somelib::diplomat::span<const somelib::MyString*> sl, W& writeable) {
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteTrait<W>::Construct(writeable);
    somelib::capi::MyString_slice_of_opaques({reinterpret_cast<const somelib::capi::MyString**>(sl.data()), sl.size()},
        &write);
}

inline std::string somelib::MyString::optional_slice_of_opaques(somelib::diplomat::span<const somelib::MyString*> sl) {
    std::string output;
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteFromString(output);
    somelib::capi::MyString_optional_slice_of_opaques({reinterpret_cast<const somelib::capi::MyString**>(sl.data()), sl.size()},
        &write);
    return output;
}
template<typename W>
inline void somelib::MyString::optional_slice_of_opaques_write(somelib::diplomat::span<const somelib::MyString*> sl, W& writeable) {
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteTrait<W>::Construct(writeable);
    somelib::capi::MyString_optional_slice_of_opaques({reinterpret_cast<const somelib::capi::MyString**>(sl.data()), sl.size()},
        &write);
}

inline const somelib::capi::MyString* somelib::MyString::AsFFI() const {
    return reinterpret_cast<const somelib::capi::MyString*>(this);
}

inline somelib::capi::MyString* somelib::MyString::AsFFI() {
    return reinterpret_cast<somelib::capi::MyString*>(this);
}

inline const somelib::MyString* somelib::MyString::FromFFI(const somelib::capi::MyString* ptr) {
    return reinterpret_cast<const somelib::MyString*>(ptr);
}

inline somelib::MyString* somelib::MyString::FromFFI(somelib::capi::MyString* ptr) {
    return reinterpret_cast<somelib::MyString*>(ptr);
}

inline void somelib::MyString::operator delete(void* ptr) {
    somelib::capi::MyString_destroy(reinterpret_cast<somelib::capi::MyString*>(ptr));
}


#endif // SOMELIB_MyString_HPP
