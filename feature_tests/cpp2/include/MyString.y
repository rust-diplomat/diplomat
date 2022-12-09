#ifndef MyString_HPP
#define MyString_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




class MyString {
public:
	static std::unique_ptr<MyString> new_(std::string_view v);

	void set_str(std::string_view new_str);

	std::string get_str() const;

	inline capi::MyString AsFFI() {
		return reinterpret_cast::<capi::MyString>(this);
	}

	~MyString() {
		capi::MyString_destroy(AsFFI());
	}

private:
	MyString() = delete;
}





#endif // MyString_HPP
