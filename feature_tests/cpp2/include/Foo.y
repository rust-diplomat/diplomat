#ifndef Foo_H
#define Foo_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


class Bar;


class Foo {
public:
	static std::unique_ptr<Foo> new_(std::string_view x);

	std::unique_ptr<Bar> get_bar();

	static std::unique_ptr<Foo> new_static(std::string_view x);

	inline capi::Foo AsFFI() {
		return reinterpret_cast::<capi::Foo>(this);
	}

	~Foo() {
		Foo_destroy(AsFFI());
	}

private:
	Foo() = delete;
}





#endif // Foo_HPP
