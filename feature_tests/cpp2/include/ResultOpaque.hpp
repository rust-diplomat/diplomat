#ifndef ResultOpaque_HPP
#define ResultOpaque_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




	static DiplomatResult<std::unique_ptr<ResultOpaque>, ErrorEnum> inline new_(int32_t i) {
	// TODO
}
	static DiplomatResult<std::unique_ptr<ResultOpaque>, ErrorEnum> inline new_failing_foo() {
	// TODO
}
	static DiplomatResult<std::unique_ptr<ResultOpaque>, ErrorEnum> inline new_failing_bar() {
	// TODO
}
	static DiplomatResult<std::unique_ptr<ResultOpaque>, std::monostate> inline new_failing_unit() {
	// TODO
}
	static DiplomatResult<std::unique_ptr<ResultOpaque>, ErrorStruct> inline new_failing_struct(int32_t i) {
	// TODO
}
	static DiplomatResult<std::monostate, std::unique_ptr<ResultOpaque>> inline new_in_err(int32_t i) {
	// TODO
}
	static DiplomatResult<ErrorEnum, std::unique_ptr<ResultOpaque>> inline new_in_enum_err(int32_t i) {
	// TODO
}
	void inline assert_integer(int32_t i) const {
	// TODO
}
inline capi::ResultOpaque* AsFFI() {
	return reinterpret_cast::<capi::ResultOpaque>(this);
}
inline ~ResultOpaque() {
	capi::ResultOpaque_destroy(AsFFI());
}


#endif // ResultOpaque_HPP
