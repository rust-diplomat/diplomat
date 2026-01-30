#ifndef SOMELIB_MyString_D_HPP
#define SOMELIB_MyString_D_HPP

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
namespace capi { struct MyString; }
class MyString;
} // namespace somelib



namespace somelib {
namespace capi {
    struct MyString;
} // namespace capi
} // namespace

namespace somelib {
class MyString {
public:

  inline static std::unique_ptr<somelib::MyString> new_(std::string_view v);

  inline static somelib::diplomat::result<std::unique_ptr<somelib::MyString>, somelib::diplomat::Utf8Error> new_unsafe(std::string_view v);

  inline static std::unique_ptr<somelib::MyString> new_from_first(somelib::diplomat::span<const diplomat::string_view_for_slice> v);

  inline void set_str(std::string_view new_str);

  inline std::string get_str() const;
  template<typename W>
  inline void get_str_write(W& writeable_output) const;

  inline static std::string_view get_static_str();

  inline static somelib::diplomat::result<std::string, somelib::diplomat::Utf8Error> string_transform(std::string_view foo);
  template<typename W>
  inline static somelib::diplomat::result<std::monostate, somelib::diplomat::Utf8Error> string_transform_write(std::string_view foo, W& writeable_output);

  inline std::string_view borrow() const;

    inline const somelib::capi::MyString* AsFFI() const;
    inline somelib::capi::MyString* AsFFI();
    inline static const somelib::MyString* FromFFI(const somelib::capi::MyString* ptr);
    inline static somelib::MyString* FromFFI(somelib::capi::MyString* ptr);
    inline static void operator delete(void* ptr);
private:
    MyString() = delete;
    MyString(const somelib::MyString&) = delete;
    MyString(somelib::MyString&&) noexcept = delete;
    MyString operator=(const somelib::MyString&) = delete;
    MyString operator=(somelib::MyString&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_MyString_D_HPP
