#ifndef MyString_D_HPP
#define MyString_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef struct MyString MyString;
}

class MyString {
public:

  inline static std::unique_ptr<MyString> new_(std::string_view v);

  inline static diplomat::result<std::unique_ptr<MyString>, diplomat::Utf8Error> new_unsafe(std::string_view v);

  inline static std::unique_ptr<MyString> new_owned(std::string_view v);

  inline void set_str(std::string_view new_str);

  inline std::string get_str() const;

  inline std::string_view get_boxed_str() const;

  inline const capi::MyString* AsFFI() const;
  inline capi::MyString* AsFFI();
  inline static const MyString* FromFFI(const capi::MyString* ptr);
  inline static MyString* FromFFI(capi::MyString* ptr);
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
