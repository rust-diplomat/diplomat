#ifndef ResultOpaque_HPP
#define ResultOpaque_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ResultOpaque.d.hpp"
#include "ResultOpaque.h"




static DiplomatResult<std::unique_ptr<ResultOpaque>, ErrorEnum> inline ResultOpaque::new_(int32_t i) {
	// TODO
}
static DiplomatResult<std::unique_ptr<ResultOpaque>, ErrorEnum> inline ResultOpaque::new_failing_foo() {
	// TODO
}
static DiplomatResult<std::unique_ptr<ResultOpaque>, ErrorEnum> inline ResultOpaque::new_failing_bar() {
	// TODO
}
static DiplomatResult<std::unique_ptr<ResultOpaque>, std::monostate> inline ResultOpaque::new_failing_unit() {
	// TODO
}
static DiplomatResult<std::unique_ptr<ResultOpaque>, ErrorStruct> inline ResultOpaque::new_failing_struct(int32_t i) {
	// TODO
}
static DiplomatResult<std::monostate, std::unique_ptr<ResultOpaque>> inline ResultOpaque::new_in_err(int32_t i) {
	// TODO
}
static DiplomatResult<ErrorEnum, std::unique_ptr<ResultOpaque>> inline ResultOpaque::new_in_enum_err(int32_t i) {
	// TODO
}
void inline ResultOpaque::assert_integer(int32_t i) const {
	// TODO
}
inline capi::ResultOpaque* ResultOpaque::AsFFI() {
	return reinterpret_cast<capi::ResultOpaque*>(this);
}
inline ResultOpaque::~ResultOpaque() {
	capi::ResultOpaque_destroy(AsFFI());
}


#endif // ResultOpaque_HPP
