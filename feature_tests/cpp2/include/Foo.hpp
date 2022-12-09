#ifndef Foo_HPP
#define Foo_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "Foo.d.hpp"
#include "Foo.h"




static std::unique_ptr<Foo> inline Foo::new_(std::string_view x) {
	// TODO
}
std::unique_ptr<Bar> inline Foo::get_bar() const {
	// TODO
}
static std::unique_ptr<Foo> inline Foo::new_static(std::string_view x) {
	// TODO
}
BorrowedFieldsReturning inline Foo::as_returning() const {
	// TODO
}
static std::unique_ptr<Foo> inline Foo::extract_from_fields(BorrowedFields fields) {
	// TODO
}
inline capi::Foo* Foo::AsFFI() {
	return reinterpret_cast<capi::Foo*>(this);
}
inline Foo::~Foo() {
	capi::Foo_destroy(AsFFI());
}


#endif // Foo_HPP
