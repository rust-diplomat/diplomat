#ifndef MyString_HPP
#define MyString_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.h"
#include "MyString.d.hpp"
#include "MyString.h"




inline std::unique_ptr<MyString> MyString::new_(std::string_view v) {
	// TODO
}
inline void MyString::set_str(std::string_view new_str) {
	// TODO
}
inline std::string MyString::get_str() const {
	// TODO
}
inline capi::MyString* MyString::AsFFI() {
	return reinterpret_cast<capi::MyString*>(this);
}
inline MyString::~MyString() {
	capi::MyString_destroy(AsFFI());
}


#endif // MyString_HPP
