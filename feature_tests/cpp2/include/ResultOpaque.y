#ifndef ResultOpaque_HPP
#define ResultOpaque_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ErrorEnum.hpp"
#include "ErrorStruct.hpp"




class ResultOpaque {
public:
	static DiplomatResult<std::unique_ptr<ResultOpaque>, ErrorEnum> new_(int32_t i);

	static DiplomatResult<std::unique_ptr<ResultOpaque>, ErrorEnum> new_failing_foo();

	static DiplomatResult<std::unique_ptr<ResultOpaque>, ErrorEnum> new_failing_bar();

	static DiplomatResult<std::unique_ptr<ResultOpaque>, std::monostate> new_failing_unit();

	static DiplomatResult<std::unique_ptr<ResultOpaque>, ErrorStruct> new_failing_struct(int32_t i);

	static DiplomatResult<std::monostate, std::unique_ptr<ResultOpaque>> new_in_err(int32_t i);

	static DiplomatResult<ErrorEnum, std::unique_ptr<ResultOpaque>> new_in_enum_err(int32_t i);

	void assert_integer(int32_t i) const;

	inline capi::ResultOpaque AsFFI() {
		return reinterpret_cast::<capi::ResultOpaque>(this);
	}

	~ResultOpaque() {
		ResultOpaque_destroy(AsFFI());
	}

private:
	ResultOpaque() = delete;
}





#endif // ResultOpaque_HPP
