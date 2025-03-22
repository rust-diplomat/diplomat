#ifndef MyString_D_HPP
#define MyString_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    struct MyString;
} // namespace capi
} // namespace

class MyString {
public:

  inline static std::unique_ptr<MyString> new_(std::string_view v);

  inline static diplomat::result<std::unique_ptr<MyString>, diplomat::Utf8Error> new_unsafe(std::string_view v);

  inline static std::unique_ptr<MyString> new_owned(std::string_view v);

  inline static std::unique_ptr<MyString> new_from_first(diplomat::span<const std::string_view> v);

  inline void set_str(std::string_view new_str);

  inline std::string get_str() const;

  inline static std::string_view get_static_str();

  inline static diplomat::result<std::string, diplomat::Utf8Error> string_transform(std::string_view foo);

  inline std::string_view borrow() const;

  inline const diplomat::capi::MyString* AsFFI() const;
  inline diplomat::capi::MyString* AsFFI();
  inline static const MyString* FromFFI(const diplomat::capi::MyString* ptr);
  inline static MyString* FromFFI(diplomat::capi::MyString* ptr);
  inline static void operator delete(void* ptr);
private:
  MyString() = delete;
  MyString(const MyString&) = delete;
  MyString(MyString&&) noexcept = delete;
  MyString operator=(const MyString&) = delete;
  MyString operator=(MyString&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // MyString_D_HPP
