#ifndef ResultOpaque_D_HPP
#define ResultOpaque_D_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.h"
#include "ErrorEnum.d.hpp"
#include "ErrorStruct.d.hpp"
#include "ResultOpaque.d.h"




class ResultOpaque {
public:
	inline static DiplomatResult<std::unique_ptr<ResultOpaque>, ErrorEnum> new_(int32_t i);

	inline static DiplomatResult<std::unique_ptr<ResultOpaque>, ErrorEnum> new_failing_foo();

	inline static DiplomatResult<std::unique_ptr<ResultOpaque>, ErrorEnum> new_failing_bar();

	inline static DiplomatResult<std::unique_ptr<ResultOpaque>, std::monostate> new_failing_unit();

	inline static DiplomatResult<std::unique_ptr<ResultOpaque>, ErrorStruct> new_failing_struct(int32_t i);

	inline static DiplomatResult<std::monostate, std::unique_ptr<ResultOpaque>> new_in_err(int32_t i);

	inline static DiplomatResult<ErrorEnum, std::unique_ptr<ResultOpaque>> new_in_enum_err(int32_t i);

	inline void assert_integer(int32_t i) const;

	inline capi::ResultOpaque* AsFFI();

	inline ~ResultOpaque();

private:
	ResultOpaque() = delete;
};





#endif // ResultOpaque_D_HPP
