#ifndef Foo_D_HPP
#define Foo_D_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "BorrowedFields.d.hpp"
#include "BorrowedFieldsReturning.d.hpp"
#include "Foo.d.h"


class Bar;


class Foo {
public:
	static std::unique_ptr<Foo> inline new_(std::string_view x);

	std::unique_ptr<Bar> inline get_bar() const;

	static std::unique_ptr<Foo> inline new_static(std::string_view x);

	BorrowedFieldsReturning inline as_returning() const;

	static std::unique_ptr<Foo> inline extract_from_fields(BorrowedFields fields);

	inline capi::Foo* AsFFI();

	inline ~Foo();

private:
	Foo() = delete;
};





#endif // Foo_D_HPP
