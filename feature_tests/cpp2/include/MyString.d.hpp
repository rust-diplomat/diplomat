#ifndef MyString_D_HPP
#define MyString_D_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "MyString.d.h"




class MyString {
public:
	static std::unique_ptr<MyString> inline new_(std::string_view v);

	void inline set_str(std::string_view new_str);

	std::string inline get_str() const;

	inline capi::MyString* AsFFI();

	inline ~MyString();

private:
	MyString() = delete;
};





#endif // MyString_D_HPP
