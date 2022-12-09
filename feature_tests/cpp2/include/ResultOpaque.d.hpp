#ifndef ResultOpaque_D_HPP
#define ResultOpaque_D_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ErrorEnum.d.hpp"
#include "ErrorStruct.d.hpp"
#include "ResultOpaque.d.h"




class ResultOpaque {
public:
	static DiplomatResult<std::unique_ptr<ResultOpaque>, ErrorEnum> inline new_(int32_t i);

	static DiplomatResult<std::unique_ptr<ResultOpaque>, ErrorEnum> inline new_failing_foo();

	static DiplomatResult<std::unique_ptr<ResultOpaque>, ErrorEnum> inline new_failing_bar();

	static DiplomatResult<std::unique_ptr<ResultOpaque>, std::monostate> inline new_failing_unit();

	static DiplomatResult<std::unique_ptr<ResultOpaque>, ErrorStruct> inline new_failing_struct(int32_t i);

	static DiplomatResult<std::monostate, std::unique_ptr<ResultOpaque>> inline new_in_err(int32_t i);

	static DiplomatResult<ErrorEnum, std::unique_ptr<ResultOpaque>> inline new_in_enum_err(int32_t i);

	void inline assert_integer(int32_t i) const;

	inline capi::ResultOpaque* AsFFI();

	inline ~ResultOpaque();

private:
	ResultOpaque() = delete;
};





#endif // ResultOpaque_D_HPP
