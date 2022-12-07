#ifndef ICU4XFixedDecimal_H
#define ICU4XFixedDecimal_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




class ICU4XFixedDecimal {
public:
	static std::unique_ptr<ICU4XFixedDecimal> new_(int32_t v);

	void multiply_pow10(int16_t power);

	void negate();

	DiplomatResult<std::string, void> to_string();

	inline capi::ICU4XFixedDecimal AsFFI() {
		return reinterpret_cast::<capi::ICU4XFixedDecimal>(this);
	}

	~ICU4XFixedDecimal() {
		ICU4XFixedDecimal_destroy(AsFFI());
	}

private:
	ICU4XFixedDecimal() = delete;
}





#endif // ICU4XFixedDecimal_HPP
