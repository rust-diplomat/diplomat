#ifndef ResultOpaque_HPP
#define ResultOpaque_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.h"
#include "ResultOpaque.d.hpp"
#include "ResultOpaque.h"




inline DiplomatResult<std::unique_ptr<ResultOpaque>, ErrorEnum> ResultOpaque::new_(int32_t i) {
	// TODO
}
inline DiplomatResult<std::unique_ptr<ResultOpaque>, ErrorEnum> ResultOpaque::new_failing_foo() {
	// TODO
}
inline DiplomatResult<std::unique_ptr<ResultOpaque>, ErrorEnum> ResultOpaque::new_failing_bar() {
	// TODO
}
inline DiplomatResult<std::unique_ptr<ResultOpaque>, std::monostate> ResultOpaque::new_failing_unit() {
	// TODO
}
inline DiplomatResult<std::unique_ptr<ResultOpaque>, ErrorStruct> ResultOpaque::new_failing_struct(int32_t i) {
	// TODO
}
inline DiplomatResult<std::monostate, std::unique_ptr<ResultOpaque>> ResultOpaque::new_in_err(int32_t i) {
	// TODO
}
inline DiplomatResult<ErrorEnum, std::unique_ptr<ResultOpaque>> ResultOpaque::new_in_enum_err(int32_t i) {
	// TODO
}
inline void ResultOpaque::assert_integer(int32_t i) const {
	// TODO
}
inline const capi::ResultOpaque* ResultOpaque::AsFFI() const {
	return reinterpret_cast<const capi::ResultOpaque*>(this);
}
inline capi::ResultOpaque* ResultOpaque::AsFFI() {
	return reinterpret_cast<capi::ResultOpaque*>(this);
}
inline ResultOpaque::~ResultOpaque() {
	capi::ResultOpaque_destroy(AsFFI());
}


#endif // ResultOpaque_HPP
