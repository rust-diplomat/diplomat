#ifndef MyString_HPP
#define MyString_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "MyString.d.hpp"
#include "MyString.h"




static std::unique_ptr<MyString> inline MyString::new_(std::string_view v) {
	// TODO
}
void inline MyString::set_str(std::string_view new_str) {
	// TODO
}
std::string inline MyString::get_str() const {
	// TODO
}
inline capi::MyString* MyString::AsFFI() {
	return reinterpret_cast<capi::MyString*>(this);
}
inline MyString::~MyString() {
	capi::MyString_destroy(AsFFI());
}


#endif // MyString_HPP
