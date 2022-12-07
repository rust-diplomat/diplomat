#ifndef ResultOpaque_HPP
#define ResultOpaque_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




class ResultOpaque {
public:
	static DiplomatResult<box_ResultOpaque, ErrorEnum> new_(int32_t i);

	static DiplomatResult<box_ResultOpaque, ErrorEnum> new_failing_foo();

	static DiplomatResult<box_ResultOpaque, ErrorEnum> new_failing_bar();

	static DiplomatResult<box_ResultOpaque, void> new_failing_unit();

	static DiplomatResult<box_ResultOpaque, ErrorStruct> new_failing_struct(int32_t i);

	static DiplomatResult<void, box_ResultOpaque> new_in_err(int32_t i);

	static DiplomatResult<ErrorEnum, box_ResultOpaque> new_in_enum_err(int32_t i);

	void assert_integer(int32_t i);

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
