#ifndef Foo_HPP
#define Foo_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




	static std::unique_ptr<Foo> inline new_(std::string_view x) {
	// TODO
}
	std::unique_ptr<Bar> inline get_bar() const {
	// TODO
}
	static std::unique_ptr<Foo> inline new_static(std::string_view x) {
	// TODO
}
	BorrowedFieldsReturning inline as_returning() const {
	// TODO
}
	static std::unique_ptr<Foo> inline extract_from_fields(BorrowedFields fields) {
	// TODO
}
inline capi::Foo* AsFFI() {
	return reinterpret_cast::<capi::Foo>(this);
}
inline ~Foo() {
	capi::Foo_destroy(AsFFI());
}


#endif // Foo_HPP
