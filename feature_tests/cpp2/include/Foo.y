#ifndef Foo_HPP
#define Foo_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "BorrowedFields.y"
#include "BorrowedFieldsReturning.y"


class Bar;


class Foo {
public:
	static std::unique_ptr<Foo> new_(std::string_view x);

	std::unique_ptr<Bar> get_bar() const;

	static std::unique_ptr<Foo> new_static(std::string_view x);

	BorrowedFieldsReturning as_returning() const;

	static std::unique_ptr<Foo> extract_from_fields(BorrowedFields fields);

	inline capi::Foo AsFFI() {
		return reinterpret_cast::<capi::Foo>(this);
	}

	~Foo() {
		capi::Foo_destroy(AsFFI());
	}

private:
	Foo() = delete;
}





#endif // Foo_HPP
