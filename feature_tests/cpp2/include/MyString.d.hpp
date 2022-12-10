#ifndef MyString_D_HPP
#define MyString_D_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "MyString.d.h"




class MyString {
public:

  inline static std::unique_ptr<MyString> new_(std::string_view v);

  inline void set_str(std::string_view new_str);

  inline std::string get_str() const;

  inline const capi::MyString* AsFFI() const;
  inline capi::MyString* AsFFI();
  inline static const MyString* FromFFI(const capi::MyString* ptr);
  inline static MyString* FromFFI(capi::MyString* ptr);
  inline ~MyString();
private:
  MyString() = delete;
};





#endif // MyString_D_HPP
